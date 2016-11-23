extern crate spotify;
use spotify::{Spotify, SpotifyError};

fn main() {
    let spotify = match Spotify::new() {
        Ok(result) => result,
        Err(error) => {
            match error {
                SpotifyError::ClientNotRunning => {
                    println!("The Spotify Client is not running!");
                    std::process::exit(1);
                }
                SpotifyError::WebHelperNotRunning => {
                    println!("The SpotifyWebHelper process is not running!");
                    std::process::exit(2);
                }
                SpotifyError::InternalError(err) => {
                    println!("Internal Error: {:?}", err);
                    std::process::exit(3);
                }
            }
        }
    };
    let status = match spotify.get_status() {
        Ok(result) => result,
        Err(error) => {
            println!("Unable to retrieve the Spotify status.\nError: {:?}", error);
            std::process::exit(4);
        }
    };
    println!("Spotify Client (Version {version})",
             version = status.client_version);
}