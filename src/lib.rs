#![warn(missing_docs)]
//! The Spotify crate.
//!
//! This crate contains methods to retrieve information from
//! and manipulate the local Spotify client instance.
//!
//!   ## What can I do with it
//!
//!   Features:
//!   - Play a track using track ID or URI
//!   - Pause/resume the currently played track
//!   - Get currently played track (including album, artist, etc.)
//!   - Get current volume
//!   - Get Spotify client version and online status
//!   - React to changes by long polling in a separate thread
//!
//!   And a few goodies:
//!   - Automatically fixes broken (but recoverable) track IDs and URIs
//!   - Check whether SpotifyWebHelper is running (Windows only)
//!
//   ## Is the project still alive?
//
//   I haven't really worked on this library for a long time, but that's mostly because it's pretty much done. There aren't any significant bugs that I know of and the cli should work just fine. I've made an effort to port everything over from 2015 to 2021 edition and the code compiles (tested using Rust 1.64).
//
//!   ## Examples
//!   The following is a minimal example to show you what's possible with spotify-rs.   
//!   Please note that this example lacks proper error handling for the sake of brevity.
//!
//!   ```rust,no_run
//!   extern crate spotify;
//!   use spotify::Spotify;
//!
//!   fn main() {
//!       // Grab an instance of the Spotify API
//!       let spotify = Spotify::connect().unwrap();
//!
//!       // Fetch the current status from Spotify
//!       let status = spotify.status().unwrap();
//!
//!       // Display the Spotify Client version
//!       println!("Spotify Client (Version {})", status.version());
//!                
//!       // Display the currently playing track
//!       println!("Playing: {:#}", status.track());
//!   }
//!   ```
//!
//!   Example output:
//!
//!   ```
//!   Spotify Client (Version 1.0.42.151.g19de0aa6)
//!   Playing: Rick Astley - Never Gonna Give You Up
//!   ```
//!
//!   Here's a complete example with long polling and better error handling:
//!
//!   ```rust,no_run
//!   extern crate spotify;
//!   use spotify::{Spotify, SpotifyError};
//!
//!   fn main() {
//!       // Grab an instance of the Spotify API.
//!       let spotify = match Spotify::connect() {
//!           Ok(result) => result,
//!           Err(error) => {
//!               // Display a nice end-user-friendly error message
//!               match error {
//!                   SpotifyError::ClientNotRunning => {
//!                       println!("The Spotify Client is not running!");
//!                       std::process::exit(1);
//!                   }
//!                   SpotifyError::WebHelperNotRunning => {
//!                       println!("The SpotifyWebHelper process is not running!");
//!                       std::process::exit(2);
//!                   }
//!                   SpotifyError::InternalError(err) => {
//!                       println!("Internal Error: {:?}", err);
//!                       std::process::exit(3);
//!                   }
//!               }
//!           }
//!       };
//!  
//!
//!       // Start polling.
//!       // Updates the state every 250ms.
//!       //
//!       // The 'status' variable holds the `SpotifyStatus`,
//!       // the 'change' variable contains booleans to indicate which fields
//!       // had changed since the last update.
//!       let reactor = spotify.poll(|_, status, change| {
//!           // Print the Spotify Client version on change.
//!           if change.client_version {
//!               println!("Spotify Client (Version {})", status.version());
//!           }
//!           // Print the currently playing track on change.
//!           if change.track {
//!               println!("Now playing: {:#}", status.track());
//!           }
//!           // Print the current volume on change.
//!           if change.volume {
//!               println!("Volume: {}%", status.volume_percentage());
//!           }
//!       
//!           // Returning true will continue polling, whereas returning
//!           // false will stop polling and return from the thread.
//!           true
//!       });
//!       
//!       // Join the reactor thread so the application
//!       // doesn't close before receiving any data.
//!       if reactor.join().ok().is_none() {
//!           println!("Unable to join into the live-update.");
//!           std::process::exit(4);
//!       }
//!   }
//!
//!   ```
//!
//!   Example output:
//!
//!   ```
//!   Spotify Client (Version 1.0.42.151.g19de0aa6)
//!   Now playing: Tim Minchin - White Wine In The Sun
//!   Volume: 100%
//!   Now playing: Tim Minchin - Encore
//!   Volume: 50%
//!   Volume: 76%
//!   Now playing: Tim Minchin - Ready For This ?
//!   Volume: 100%
//!   ```
//!
//!   ## F.A.Q.
//!   **It doesn't connect, what's wrong?**    
//!   Make sure that Spotify is running and the SpotifyWebHelper process is active.
//!
//!   If you can't find SpotifyWebHelper.exe in your process list, you might have disabled it by accident. Here's how you enable it:
//!
//!   - Open Spotify
//!   - Press `Ctrl` + `P` to open the preferences
//!   - Scroll down and click 'Show advanced settings'
//!   - In the `Startup and Window Behaviour` section,   
//!     enable `Allow Spotify to be opened from the web`.
//!
//!   You might wanna restart Spotify after doing that.   
//!
//!   > **Update**: I'm not sure if this option is still exposed nowadays. Spotify 1.1.95 (2022) on macOS doesn't seem to have this anymore, and I'm not sure if Spotify still exposes the local API at all. If it doesn't, this library is pretty much useless. If you know whether this still works, please open an issue and let me know!

// Extern crates
extern crate json;
extern crate reqwest;
extern crate time;
extern crate winapi;

// Modules
mod connector;
pub mod status;
#[cfg(windows)]
mod windows_process;

// Imports
use crate::connector::{InternalSpotifyError, SpotifyConnector};
use crate::status::{SpotifyStatus, SpotifyStatusChange};
use std::thread::{self, JoinHandle};
use std::time::Duration;
#[cfg(windows)]
use windows_process::WindowsProcess;

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
    pub fn poll<F: 'static>(self, f: F) -> JoinHandle<()>
    where
        F: Fn(&Spotify, SpotifyStatus, SpotifyStatusChange) -> bool,
        F: std::marker::Send,
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
                .trim_start_matches("http://") // get rid of protocol
                .trim_start_matches("open.spotify.com") // get rid of domain name
                .replace('/', ":") // turn all / into :
                .trim_start_matches(':') // get rid of : at the beginning
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
