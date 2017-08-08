extern crate spotify;
use spotify::{Spotify, SpotifyError};
use std::{thread, time};

fn main() {
    let spotify = match Spotify::connect() {
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
    let reactor = spotify.poll(|client, status, change| {
        if change.client_version {
            println!("Spotify Client (Version {})", status.version());
        }
        if change.track {
            println!("Now playing: {:#}", status.track());
            println!("{}", status.full_track().track.uri);
        }
        true
    });
    if reactor.join().ok().is_none() {
        println!("Unable to join into the live-update.");
        std::process::exit(4);
    }
}