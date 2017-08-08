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
    /// Connects to the local Spotify client.
    #[cfg(windows)]
    pub fn connect() -> Result<Spotify> {
        // TODO:
        // At some point, the connector should automatically
        // open Spotify in the case  that Spotify is closed.
        // That would also be a much better cross-platform solution,
        // because it would work on Linux and macOS and make
        // the dependency on winapi and kernel32-sys unnecessary.
        if !Spotify::spotify_webhelper_alive() {
            return Err(SpotifyError::WebHelperNotRunning);
        }
        Spotify::new_unchecked()
    }
    /// Connects to the local Spotify client.
    #[cfg(not(windows))]
    pub fn connect() -> Result<Spotify> {
        Spotify::new_unchecked()
    }
    /// Constructs a new `self::Result<Spotify>`.
    fn new_unchecked() -> Result<Spotify> {
        match SpotifyConnector::connect_new() {
            Ok(result) => Ok(Spotify { connector: result }),
            Err(error) => Err(SpotifyError::InternalError(error)),
        }
    }
    /// Moves `self` to a new thread and begins polling the
    /// client status. Sends the updated status to the specified
    /// closure, together with information of which fields had changed
    /// since the last update. Returns the `JoinHandle` of the new thread.
    pub fn poll<'a, F: 'static>(self, f: F) -> JoinHandle<()>
        where F: Fn(&Spotify, SpotifyStatus, SpotifyStatusChange) -> bool,
              F: std::marker::Send
    {
        thread::spawn(move || {
            let sleep_time = Duration::from_millis(250);
            let mut last: Option<SpotifyStatus> = None;
            let mut curr: Option<SpotifyStatus>;
            let mut first = true;
            loop {
                curr = get_status(&self.connector).ok();
                {
                    let last = last.clone();
                    if first && curr.is_some() {
                        let curr = curr.clone().unwrap();
                        if !f(&self, curr.clone(), SpotifyStatusChange::new_true()) {
                            break;
                        }
                        first = false;
                    } else if !first && curr.is_some() && last.is_some() {
                        let curr = curr.clone().unwrap();
                        let last = last.unwrap();
                        if !f(&self, curr.clone(), SpotifyStatusChange::from((curr, last))) {
                            break;
                        }
                    }
                }
                if curr.is_some() {
                    last = curr.clone();
                }
                thread::sleep(sleep_time);
            }
        })
    }
    /// Fetches the current status from the client.
    pub fn status(&self) -> Result<SpotifyStatus> {
        get_status(&self.connector)
    }
    /// Plays a track.
    pub fn play(&self, track: String) -> bool {
        // Try to fix broken track URIs
        // In: https://open.spotify.com/track/1pGZIV8olkbRMjyHWoEXyt
        // In: open.spotify.com/track/1pGZIV8olkbRMjyHWoEXyt
        // In: track/1pGZIV8olkbRMjyHWoEXyt
        // In: track:1pGZIV8olkbRMjyHWoEXyt
        // Out: spotify:track:1pGZIV8olkbRMjyHWoEXyt
        let track: String = {
            let track = track
                .replace("https://", "http://") // https -> http
                .trim_left_matches("http://") // get rid of protocol
                .trim_left_matches("open.spotify.com") // get rid of domain name
                .replace("/", ":") // turn all / into :
                .trim_left_matches(":") // get rid of : at the beginning
                .to_owned();
            if track.starts_with("spotify:") {
                track
            } else {
                format!("spotify:{}", track) // prepend proper protocol
            }
        };
        // Play the track
        self.connector.request_play(track)
    }
    /// Pauses the currently playing track.
    /// Has no effect if the track is already paused.
    pub fn pause(&self) -> bool {
        self.connector.request_pause(true)
    }
    /// Resumes the currently paused track.
    /// Has no effect if the track is already playing.
    pub fn resume(&self) -> bool {
        self.connector.request_pause(false)
    }
    /// Tests whether the SpotifyWebHelper process is running.
    #[cfg(windows)]
    fn spotify_webhelper_alive() -> bool {
        let process = "SpotifyWebHelper.exe";
        WindowsProcess::find_by_name(process).is_some()
    }
}