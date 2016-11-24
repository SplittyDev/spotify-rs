# Spotify-rs
[![Travis CI](https://img.shields.io/travis/SplittyDev/spotify-rs/master.svg?style=flat-square)][travis-url]
[![Crates.io](https://img.shields.io/crates/l/spotify.svg?style=flat-square)][crates-url]
[![Crates.io](https://img.shields.io/crates/v/spotify.svg?style=flat-square)][crates-url]

Easy to use Spotify Local API abstraction library.

## What it is
Spotify-rs provides an easy-to-use abstraction over the Spotify Local API.   
It is made for fetching information from the local Spotify Client.

You can easily retrieve the currently playing track, the artist who made it,   
the album it's from, the version of the Spotify Client, whether the Spotify Client   
is online or offline, etc.

## What is isn't
Spotify-rs isn't some kind of hack. It just uses Spotify's own local server.   
It only allows fetching information from the client and, maybe in the future,   
sending stuff back to the client (e.g. make Spotify play a specific track).

## Examples
The following is a minimal example to show you what's possible with spotify-rs.   
Please note that this example code lacks error checking, it's really only a quick demonstration.
```rust
extern crate spotify;
use spotify::Spotify;

fn main() {
    // Grab an instance of the Spotify API
    let spotify = Spotify::new().unwrap();

    // Fetch the current status from Spotify
    let status = match spotify.get_status();

    // Display the Spotify Client version
    println!("Spotify Client (Version {})", status.client_version);
             
    // Display the currently playing track
    println!("Playing: '{track}' by '{artist}' ({album})",
             track = status.track.track.name,
             album = status.track.album.name,
             artist = status.track.artist.name,
    );
}
```

For the sake of completeness I also have a complete example for you,   
with proper error checking, which does the same as the example above, but safe! :P

I made error handling really easy, all you need to do is match on the error and   
provide a few end-user-friendly clues. That's all!
```rust
extern crate spotify;
use spotify::{Spotify, SpotifyError};

fn main() {
    // Grab an instance of the Spotify API.
    let spotify = match Spotify::new() {
        Ok(result) => result,
        Err(error) => {
            // Display a nice end-user-friendly error message
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

    // Fetch the current status from Spotify
    let status = match spotify.get_status() {
        Ok(result) => result,
        Err(error) => {
            println!("Unable to retrieve the Spotify status.\nError: {:?}", error);
            std::process::exit(4);
        }
    };

    // Display the Spotify Client version
    println!("Spotify Client (Version {})", status.client_version);
            
    // Display the currently playing track
    match status.track {
        Some(res) => {
            println!("Playing: '{track}' by '{artist}' ({album})",
                track = res.track.name,
                album = res.album.name,
                artist = res.artist.name,
            );
        }
        None => println!("No track is currently playing."),
    };
}
```

## F.A.Q.
**It doesn't connect, what's wrong?**   
You probably forgot to start Spotify.   
Also make sure that the SpotifyWebHelper process is active.

If you can't find SpotifyWebHelper.exe in your process list,   
you might have it disabled. Here's how you enable it:

- Open Spotify
- Press `Ctrl` + `P` to open the preferences
- Scroll down and click 'Show advanced settings'
- In the `Startup and Window Behaviour` section,   
  enable `Allow Spotify to be opened from the web`.

You might wanna restart Spotify after doing that.   
That's it! Everything should work now.

[travis-url]: https://travis-ci.org/SplittyDev/spotify-rs
[crates-url]: https://crates.io/crates/spotify
