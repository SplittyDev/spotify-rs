use std::io::Read;
use std::sync::Mutex;
use reqwest::{self, Client};
use reqwest::header::{Origin, Referer, UserAgent};
use json::{self, JsonValue};
use time;

// Headers
const HEADER_UA: &'static str = "Mozilla/5.0 (Windows; rv:50.0) Gecko/20100101 Firefox/50.0";
const HEADER_ORIGIN_SCHEME: &'static str = "https";
const HEADER_ORIGIN_HOST: &'static str = "embed.spotify.com";

// Spotify base URLs
const SPOTIFY_URL_EMBED: &'static str = "https://embed.spotify.com";
const SPOTIFY_URL_TOKEN: &'static str = "https://open.spotify.com/token";
const SPOTIFY_URL_LOCAL: &'static str = "https://spotifyrs.spotilocal.com:4371";

// Spotify request end-points
const REQUEST_CSRF: &'static str = "simplecsrf/token.json";
const REQUEST_STATUS: &'static str = "remote/status.json";
const REQUEST_PLAY: &'static str = "remote/play.json";
#[allow(dead_code)]
const REQUEST_OPEN: &'static str = "open.json";

// The referal track
const REFERAL_TRACK: &'static str = "track/4uLU6hMCjMI75M1A2tKUQC";

/// The `Result` type used in this module.
type Result<T> = ::std::result::Result<T, InternalSpotifyError>;

/// The `InternalSpotifyError` enum.
#[derive(Debug)]
pub enum InternalSpotifyError {
    // Reqwest
    ReqwestError(reqwest::Error),
    // JSON
    JSONParseError(json::Error),
    // OAUth
    InvalidOAuthToken,
    // CSRF
    InvalidCSRFToken,
    // Other
    IOError(::std::io::Error),
}

/// The `SpotifyConnector` struct.
pub struct SpotifyConnector {
    /// The Reqwest client.
    client: Mutex<Client>,
    /// The Spotify OAuth token.
    oauth_token: String,
    /// The Spotify CSRF token.
    csrf_token: String,
}

/// Implements `SpotifyConnector`.
impl SpotifyConnector {
    /// Constructs a new `SpotifyConnector`.
    /// Retrieves the OAuth and CSRF tokens in the process.
    pub fn connect_new() -> Result<SpotifyConnector> {
        let client = match Client::new() {
            Ok(client) => client,
            Err(error) => return Err(InternalSpotifyError::ReqwestError(error)),
        };
        let mut connector = SpotifyConnector {
            client: Mutex::new(client),
            oauth_token: String::default(),
            csrf_token: String::default(),
        };
        connector.oauth_token = match connector.fetch_oauth_token() {
            Ok(result) => result,
            Err(error) => return Err(error),
        };
        connector.csrf_token = match connector.fetch_csrf_token() {
            Ok(result) => result,
            Err(error) => return Err(error),
        };
        Ok(connector)
    }
    /// Fetches the OAuth token from Spotify.
    fn fetch_oauth_token(&self) -> Result<String> {
        let json = match self.query(SPOTIFY_URL_TOKEN, "", false, false, None) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };
        match json["t"].as_str() {
            Some(token) => Ok(token.to_owned()),
            None => Err(InternalSpotifyError::InvalidOAuthToken),
        }
    }
    /// Fetches the CSRF token from Spotify.
    fn fetch_csrf_token(&self) -> Result<String> {
        let json = match self.query(SPOTIFY_URL_LOCAL, REQUEST_CSRF, false, false, None) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };
        match json["token"].as_str() {
            Some(token) => Ok(token.to_owned()),
            None => Err(InternalSpotifyError::InvalidCSRFToken),
        }
    }
    /// Fetches the current status from Spotify.
    pub fn fetch_status_json(&self) -> Result<JsonValue> {
        self.query(SPOTIFY_URL_LOCAL, REQUEST_STATUS, true, true, None)
    }
    /// Requests a track to be played.
    pub fn request_play(&self, track: String, queue: bool) -> bool {
        let params = {
            let mut params = Vec::<String>::new();
            if queue {
                params.push("action=queue".into());
            }
            params.push(format!("uri={0}&context={0}", track));
            Some(params)
        };
        match self.query(SPOTIFY_URL_LOCAL, REQUEST_PLAY, true, true, params) {
            Ok(_) => true,
            _ => false,
        }
    }
    /// Queries the specified base url with the specified query.
    /// Optionally includes the OAuth and/or CSRF token in the query.
    fn query(&self,
             base: &str,
             query: &str,
             with_oauth: bool,
             with_csrf: bool,
             params: Option<Vec<String>>)
             -> Result<JsonValue> {
        let timestamp = time::now_utc().to_timespec().sec;
        let arguments = {
            let mut arguments = String::new();
            if !query.contains("?") {
                arguments.push_str("?");
            }
            arguments.push_str("&ref=&cors=");
            arguments.push_str(format!("&_={}", timestamp).as_ref());
            if with_oauth {
                arguments.push_str(format!("&oauth={}", self.oauth_token).as_ref());
            }
            if with_csrf {
                arguments.push_str(format!("&csrf={}", self.csrf_token).as_ref());
            }
            if let Some(params) = params {
                for elem in params {
                    arguments.push_str(format!("&{}", elem).as_ref());
                }
            }
            arguments
        };
        let url = format!("{}/{}{}", base, query, arguments);
        let response = {
            let mut content = String::new();
            let mut resp = match self.client
                .lock()
                .unwrap()
                .get::<&str>(url.as_ref())
                .header(UserAgent(HEADER_UA.into()))
                .header(Origin::new(HEADER_ORIGIN_SCHEME, HEADER_ORIGIN_HOST, None))
                .header(Referer(format!("{}/{}", SPOTIFY_URL_EMBED, REFERAL_TRACK)))
                .send() {
                Ok(result) => result,
                Err(error) => return Err(InternalSpotifyError::ReqwestError(error)),
            };
            match resp.read_to_string(&mut content) {
                Ok(_) => content,
                Err(error) => return Err(InternalSpotifyError::IOError(error)),
            }
        };
        match json::parse(response.as_ref()) {
            Ok(result) => Ok(result),
            Err(error) => Err(InternalSpotifyError::JSONParseError(error)),
        }
    }
}