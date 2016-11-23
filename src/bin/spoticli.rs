extern crate spotify;
use spotify::{Spotify, SpotifyError};

#[allow(unused_variables)]
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
    println!("It's working!");
}