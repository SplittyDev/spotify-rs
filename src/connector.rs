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
        match Client::new() {
            Ok(client) => {
                let token_req = client.get(SPOTIFY_URL_TOKEN)
                    .header(UserAgent(USER_AGENT.into()))
                    .send();
                match token_req {
                    Ok(mut token_resp) => {
                        let mut json_str = String::new();
                        match token_resp.read_to_string(&mut json_str) {
                            Ok(_) => {
                                match self::json::parse(json_str.as_ref()) {
                                    Ok(json) => {
                                        match json["t"].as_str() {
                                            Some(token) => {
                                                Ok(SpotifyConnector {
                                                    host: host,
                                                    client: client,
                                                    token: token.into(),
                                                })
                                            }
                                            None => Err(SpotifyError::InvalidToken),
                                        }
                                    }
                                    Err(error) => Err(SpotifyError::TokenParseError(error)),
                                }
                            }
                            Err(error) => Err(SpotifyError::TokenIOError(error)),
                        }
                    }
                    Err(error) => Err(SpotifyError::ReqwestError(error)),
                }
            }
            Err(error) => Err(SpotifyError::ReqwestError(error)),
        }
    }
}