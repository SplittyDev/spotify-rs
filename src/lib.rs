mod windows_process;
mod connector;
mod webhelper;

use connector::SpotifyConnector;
// use webhelper::SpotifyWebHelper;

/// The Spotify API.
#[allow(dead_code)]
pub struct Spotify {
    connector: SpotifyConnector,
}

/// Implements `Spotify`.
impl Spotify {
    /// Constructs a new `Spotify`.
    pub fn new() -> Spotify {
        Spotify { connector: SpotifyConnector::new("127.0.0.1".into()).unwrap() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
