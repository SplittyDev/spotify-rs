#![allow(dead_code)]

use std::io::Read;
extern crate reqwest;
use self::reqwest::Client;
use self::reqwest::header::UserAgent;
extern crate json;

const USER_AGENT: &'static str = "Mozilla/5.0 (Windows; rv:50.0) Gecko/20100101 Firefox/50.0";
const SPOTIFY_URL_TOKEN: &'static str = "https://open.spotify.com/token";

/// The `SpotifyError` enum.
#[derive(Debug)]
pub enum SpotifyError {
    ReqwestError(self::reqwest::Error),
    TokenParseError(self::json::Error),
    TokenIOError(::std::io::Error),
    InvalidToken,
}

/// The `SpotifyConnector` struct.
pub struct SpotifyConnector {
    client: Client,
    token: String,
    host: String,
}

/// Implements `SpotifyConnector`.
impl SpotifyConnector {
    pub fn new(host: String) -> Result<SpotifyConnector, SpotifyError> {
        let client = match Client::new() {
            Ok(client) => client,
            Err(error) => return Err(SpotifyError::ReqwestError(error)),
        };
        let token = {
            let mut token_resp =
                match client.get(SPOTIFY_URL_TOKEN).header(UserAgent(USER_AGENT.into())).send() {
                    Ok(resp) => resp,
                    Err(err) => return Err(SpotifyError::ReqwestError(err)),
                };
            let json = {
                let mut json_str = String::new();
                if let Err(error) = token_resp.read_to_string(&mut json_str) {
                    return Err(SpotifyError::TokenIOError(error));
                }
                match self::json::parse(json_str.as_ref()) {
                    Ok(data) => data,
                    Err(err) => return Err(SpotifyError::TokenParseError(err)),
                }
            };
            match json["t"].as_str() {
                Some(token) => token.to_owned(),
                None => return Err(SpotifyError::InvalidToken),
            }
        };
        Ok(SpotifyConnector {
            host: host,
            client: client,
            token: token.into(),
        })
    }
    pub fn get_oauth_token(&self) -> String {
        self.token.clone()
    }
}