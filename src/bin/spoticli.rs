extern crate spotify;
use spotify::Spotify;

#[allow(unused_variables)]
fn main() {
    let spotify = Spotify::new();
    println!("Spotify running: {}", Spotify::spotify_alive());
    println!("Spotify WebHelper running: {}",
             Spotify::spotify_webhelper_alive());
    println!("OAuth Token: {}", spotify.connector.get_oauth_token());
}