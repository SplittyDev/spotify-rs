extern crate spotify;

use spotify::webhelper::SpotifyWebHelper;

fn main() {
    println!("Spotify Web Helper alive: {}", SpotifyWebHelper.is_alive())
}