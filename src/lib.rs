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
pub mod status;

// Imports
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::Arc;
#[cfg(windows)]
use windows_process::WindowsProcess;
use connector::{SpotifyConnector, InternalSpotifyError};
use status::{SpotifyStatus, SpotifyStatusChange};

/// The `Result` type used in this crate.
type Result<T> = std::result::Result<T, SpotifyError>;

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
pub struct Spotify {
    /// The Spotify connector.
    connector: SpotifyConnector,
}

/// Fetches the current status from Spotify.
fn get_status(connector: &SpotifyConnector) -> Result<SpotifyStatus> {
    match connector.fetch_status_json() {
        Ok(result) => Ok(SpotifyStatus::from(result)),
        Err(error) => Err(SpotifyError::InternalError(error)),
    }
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
    #[cfg(not(windows))]
    pub fn new() -> Result<Spotify> {
        Spotify::new_unchecked()
    }
    /// Constructs a new `Spotify`.
    ///
    /// Skips the checks done in `Spotify::new`.
    fn new_unchecked() -> Result<Spotify> {
        match SpotifyConnector::connect_new() {
            Ok(result) => Ok(Spotify { connector: result }),
            Err(error) => Err(SpotifyError::InternalError(error)),
        }
    }
    /// Polls the Spotify status and passes it,
    /// to the specified closure together with a structure
    /// indicating which fields changed since the last update.
    pub fn poll<F: 'static>(self, f: F) -> JoinHandle<()>
        where F: Fn(SpotifyStatus, SpotifyStatusChange) -> bool,
              F: std::marker::Send
    {
        let connector = Arc::new(self.connector);
        thread::spawn(move || {
            let sleep_time = Duration::from_millis(250);
            let mut last: Option<SpotifyStatus> = None;
            let mut curr: Option<SpotifyStatus>;
            loop {
                curr = get_status(&connector).ok();
                if curr.is_some() && last.is_none() {
                    if !f(curr.clone().unwrap(), SpotifyStatusChange::new_true()) {
                        break;
                    }
                } else if curr.is_some() && last.is_some() {
                    let curr = curr.clone().unwrap();
                    let last = last.unwrap();
                    if !f(curr.clone(), SpotifyStatusChange::from((curr, last))) {
                        break;
                    }
                }
                last = curr.clone();
                thread::sleep(sleep_time);
            }
        })
    }
    /// Fetches the current status from the Spotify client.
    pub fn get_status(&self) -> Result<SpotifyStatus> {
        get_status(&self.connector)
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