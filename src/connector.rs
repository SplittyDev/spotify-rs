#![allow(dead_code)]

use std::io::Read;
extern crate reqwest;
use self::reqwest::Client;
use self::reqwest::header::{Origin, Referer, UserAgent};
extern crate json;
extern crate time;

const HEADER_UA: &'static str = "Mozilla/5.0 (Windows; rv:50.0) Gecko/20100101 Firefox/50.0";
const HEADER_ORIGIN_SCHEME: &'static str = "https";
const HEADER_ORIGIN_HOST: &'static str = "embed.spotify.com";
const SPOTIFY_URL_EMBED: &'static str = "https://embed.spotify.com";
const SPOTIFY_URL_TOKEN: &'static str = "https://open.spotify.com/token";
const SPOTIFY_URL_LOCAL: &'static str = "https://nspotify.spotilocal.com:4371";
const REQUEST_CFID: &'static str = "simplecsrf/token.json";
const REQUEST_STATIC: &'static str = "remote/status.json";
const REFERAL_TRACK: &'static str = "track/4uLU6hMCjMI75M1A2tKUQC";

/// The `Result` type used in this module.
type Result<T> = ::std::result::Result<T, InternalSpotifyError>;

/// The `InternalSpotifyError` enum.
#[derive(Debug)]
pub enum InternalSpotifyError {
    // Reqwest
    ReqwestError(self::reqwest::Error),
    // OAUth
    OAuthTokenParseError(self::json::Error),
    InvalidOAuthToken,
    // CSRF
    CSRFTokenParseError(self::json::Error),
    CSRFTokenError(String),
    InvalidCSRFToken,
    // Other
    IOError(::std::io::Error),
}

/// The `SpotifyConnector` struct.
pub struct SpotifyConnector {
    client: Client,
    host: String,
    oauth_token: String,
    csrf_token: String,
}

/// Implements `SpotifyConnector`.
impl SpotifyConnector {
    /// Constructs a new `SpotifyConnector`.
    pub fn new(host: String) -> Result<SpotifyConnector> {
        let client = match Client::new() {
            Ok(client) => client,
            Err(error) => return Err(InternalSpotifyError::ReqwestError(error)),
        };
        let mut connector = SpotifyConnector {
            host: host,
            client: client,
            oauth_token: String::default(),
            csrf_token: String::default(),
        };
        connector.oauth_token = match connector.retrieve_oauth_token() {
            Ok(result) => result,
            Err(error) => return Err(error),
        };
        connector.csrf_token = match connector.retrieve_csrf_token() {
            Ok(result) => result,
            Err(error) => return Err(error),
        };
        Ok(connector)
    }
    /// Fetches the OAuth token from Spotify.
    fn retrieve_oauth_token(&self) -> Result<String> {
        let mut token_resp =
            match self.client.get(SPOTIFY_URL_TOKEN).header(UserAgent(HEADER_UA.into())).send() {
                Ok(resp) => resp,
                Err(err) => return Err(InternalSpotifyError::ReqwestError(err)),
            };
        let json_data = {
            let mut json_str = String::new();
            if let Err(error) = token_resp.read_to_string(&mut json_str) {
                return Err(InternalSpotifyError::IOError(error));
            }
            match self::json::parse(json_str.as_ref()) {
                Ok(data) => data,
                Err(err) => return Err(InternalSpotifyError::OAuthTokenParseError(err)),
            }
        };
        match json_data["t"].as_str() {
            Some(token) => Ok(token.to_owned()),
            None => Err(InternalSpotifyError::InvalidOAuthToken),
        }
    }
    /// Fetches the CSRF token from Spotify.
    fn retrieve_csrf_token(&self) -> Result<String> {
        let resp = match self.query(REQUEST_CFID, false, false) {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };
        let json_data = {
            match self::json::parse(resp.as_ref()) {
                Ok(data) => data,
                Err(err) => return Err(InternalSpotifyError::CSRFTokenParseError(err)),
            }
        };
        match json_data["token"].as_str() {
            Some(token) => Ok(token.to_owned()),
            None => Err(InternalSpotifyError::InvalidCSRFToken),
        }
    }
    /// Queries the local Spotify server.
    fn query(&self, request: &str, with_oauth: bool, with_cfid: bool) -> Result<String> {
        let timestamp = self::time::now_utc().to_timespec().sec;
        let arguments = {
            let delimiter = match request.contains("?") {
                false => format!("?"),
                _ => String::default(),
            };
            let oauth_param = match with_oauth {
                true => format!("&oauth={}", self.oauth_token),
                _ => String::default(),
            };
            let cfid_param = match with_cfid {
                true => format!("&cfid={}", self.csrf_token),
                _ => String::default(),
            };
            format!("{}&ref=&cors=&_={ts}{oauth}{cfid}",
                    delimiter,
                    ts = timestamp,
                    oauth = oauth_param,
                    cfid = cfid_param)
        };
        let url = format!("{}/{}{}", SPOTIFY_URL_LOCAL, request, arguments);
        let response = {
            let mut content = String::new();
            let mut resp = match self.client
                .get::<&str>(url.as_ref())
                .header(Origin::new(HEADER_ORIGIN_SCHEME, HEADER_ORIGIN_HOST, None))
                .header(Referer(format!("{}/{}", SPOTIFY_URL_EMBED, REFERAL_TRACK)))
                .send() {
                Ok(resp) => resp,
                Err(err) => return Err(InternalSpotifyError::ReqwestError(err)),
            };
            match resp.read_to_string(&mut content) {
                Ok(_) => content,
                Err(error) => return Err(InternalSpotifyError::IOError(error)),
            }
        };
        Ok(response)
    }
    pub fn get_oauth_token(&self) -> String {
        self.oauth_token.clone()
    }
    pub fn get_csrf_token(&self) -> String {
        self.csrf_token.clone()
    }
}