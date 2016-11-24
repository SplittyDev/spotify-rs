#![warn(missing_docs)]
//! The Spotify crate.
//!
//! This crate contains methods to retrieve information from
//! and manipulate the local Spotify client instance.

// Extern crates
extern crate winapi;
extern crate kernel32;
extern crate reqwest;
extern crate time;
extern crate json;

// Modules
#[cfg(windows)]
mod windows_process;
mod connector;
mod status;

// Imports
#[cfg(windows)]
use windows_process::WindowsProcess;
use connector::{SpotifyConnector, InternalSpotifyError};
use status::SpotifyStatus;
use json::JsonValue;

/// The `Result` type used in this crate.
type Result<T> = ::std::result::Result<T, SpotifyError>;

/// The `SpotifyError` enum.
#[derive(Debug)]
pub enum SpotifyError {
    /// An internal error.
    InternalError(InternalSpotifyError),
    /// Indicates that the Spotify Client is not running.
    ClientNotRunning,
    /// Indicates that the SpotifyWebHelper process it not running.
    WebHelperNotRunning,
}

/// The Spotify API.
#[allow(dead_code)]
pub struct Spotify {
    /// The Spotify connector.
    connector: SpotifyConnector,
}

/// Implements `Spotify`.
impl Spotify {
    /// Constructs a new `Spotify`.
    ///
    /// Does additional checks to verify that Spotify
    /// and SpotifyWebHelper are running.
    #[cfg(windows)]
    pub fn new() -> Result<Spotify> {
        if !Spotify::spotify_alive() {
            return Err(SpotifyError::ClientNotRunning);
        }
        if !Spotify::spotify_webhelper_alive() {
            return Err(SpotifyError::WebHelperNotRunning);
        }
        Spotify::new_unchecked()
    }
    /// Constructs a new `Spotify`.
    ///
    /// Skips the checks and calls `Spotify::new_unchecked` directly.
    #[cfg(not(windows))]
    pub fn new() -> Result<Spotify> {
        Spotify::new_unchecked()
    }
    /// Constructs a new `Spotify`.
    ///
    /// Skips the checks done in `Spotify::new`.
    pub fn new_unchecked() -> Result<Spotify> {
        match SpotifyConnector::new() {
            Ok(result) => Ok(Spotify { connector: result }),
            Err(error) => Err(SpotifyError::InternalError(error)),
        }
    }
    /// Fetches the current status from Spotify.
    pub fn get_status(&self) -> Result<SpotifyStatus> {
        let json = match self.get_status_object() {
            Ok(result) => result,
            Err(error) => return Err(error),
        };
        Ok(SpotifyStatus::from(json))
    }
    /// Fetches the current status from Spotify.
    pub fn get_status_object(&self) -> Result<JsonValue> {
        match self.connector.fetch_status_json() {
            Ok(result) => Ok(result),
            Err(error) => Err(SpotifyError::InternalError(error)),
        }
    }
    /// Tests whether the Spotify process is running.
    #[cfg(windows)]
    fn spotify_alive() -> bool {
        let process = "Spotify.exe";
        WindowsProcess::find_by_name(process).is_some()
    }
    /// Tests whether the SpotifyWebHelper process is running.
    #[cfg(windows)]
    fn spotify_webhelper_alive() -> bool {
        let process = "SpotifyWebHelper.exe";
        WindowsProcess::find_by_name(process).is_some()
    }
}