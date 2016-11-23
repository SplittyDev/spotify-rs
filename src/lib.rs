mod windows_process;
mod connector;
mod webhelper;

use connector::SpotifyConnector;
use windows_process::WindowsProcess;
// use webhelper::SpotifyWebHelper;

/// The Spotify API.
#[allow(dead_code)]
pub struct Spotify {
    pub connector: SpotifyConnector,
}

/// Implements `Spotify`.
impl Spotify {
    /// Constructs a new `Spotify`.
    pub fn new() -> Spotify {
        Spotify { connector: SpotifyConnector::new("127.0.0.1".into()).unwrap() }
    }
    /// Tests whether the Spotify process is running.
    pub fn spotify_alive() -> bool {
        let process = "Spotify.exe";
        WindowsProcess::find_by_name(process).is_some()
    }
    /// Tests whether the SpotifyWebHelper process is running.
    pub fn spotify_webhelper_alive() -> bool {
        let process = "SpotifyWebHelper.exe";
        WindowsProcess::find_by_name(process).is_some()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
