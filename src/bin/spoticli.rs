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
    let status = match spotify.get_status() {
        Ok(result) => result,
        Err(error) => {
            println!("Unable to retrieve the Spotify status.\nError: {:?}", error);
            std::process::exit(4);
        }
    };
    println!("Connected: Spotify Client (Version {version})",
             version = status["client_version"]);
    println!("Currently playing: {track} by {artist} (Album: {album})",
             track = status["track"]["track_resource"]["name"],
             artist = status["track"]["artist_resource"]["name"],
             album = status["track"]["album_resource"]["name"]);
}