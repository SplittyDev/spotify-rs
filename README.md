# Spotify-rs
[![Crates.io](https://img.shields.io/crates/l/spotify.svg?style=flat-square)][crates-url]
[![Crates.io](https://img.shields.io/crates/v/spotify.svg?style=flat-square)][crates-url]

[Documentation][docs-url]

Spotify-rs provides an easy-to-use abstraction over the Spotify Local API.

## ⚠️ DEPRECATION WARNING

This library is deprecated and no longer maintained. Spotify has removed the Local API and SpotifyWebHelper from their desktop clients, so this library is no longer useful. I'm keeping it around for historical purposes, but there's no point in using it anymore.

## What can I do with it

Features:
- Play a track using track ID or URI
- Pause/resume the currently played track
- Get currently played track (including album, artist, etc.)
- Get current volume
- Get Spotify client version and online status
- React to changes by long polling in a separate thread

And a few goodies:
- Automatically fixes broken (but recoverable) track IDs and URIs
- Check whether SpotifyWebHelper is running (Windows only)

## Is the project still alive?

I haven't really worked on this library for a long time, but that's mostly because it's pretty much done. There aren't any significant bugs that I know of and the cli should work just fine. I've made an effort to port everything over from 2015 to 2021 edition and the code compiles (tested using Rust 1.64).

## Examples
The following is a minimal example to show you what's possible with spotify-rs.   
Please note that this example lacks proper error handling for the sake of brevity.

```rust,no_run
extern crate spotify;
use spotify::Spotify;

fn main() {
    // Grab an instance of the Spotify API
    let spotify = Spotify::connect().unwrap();

    // Fetch the current status from Spotify
    let status = spotify.status().unwrap();

    // Display the Spotify Client version
    println!("Spotify Client (Version {})", status.version());
             
    // Display the currently playing track
    println!("Playing: {:#}", status.track());
}
```

Example output:

```
Spotify Client (Version 1.0.42.151.g19de0aa6)
Playing: Rick Astley - Never Gonna Give You Up
```

Here's a complete example with long polling and better error handling:

```rust,no_run
extern crate spotify;
use spotify::{Spotify, SpotifyError};

fn main() {
    // Grab an instance of the Spotify API.
    let spotify = match Spotify::connect() {
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

    // Start polling.
    // Updates the state every 250ms.
    // 
    // The 'status' variable holds the `SpotifyStatus`,
    // the 'change' variable contains booleans to indicate which fields
    // had changed since the last update.
    let reactor = spotify.poll(|_, status, change| {
        // Print the Spotify Client version on change.
        if change.client_version {
            println!("Spotify Client (Version {})", status.version());
        }
        // Print the currently playing track on change.
        if change.track {
            println!("Now playing: {:#}", status.track());
        }
        // Print the current volume on change.
        if change.volume {
            println!("Volume: {}%", status.volume_percentage());
        }

        // Returning true will continue polling, whereas returning
        // false will stop polling and return from the thread.
        true
    });

    // Join the reactor thread so the application
    // doesn't close before receiving any data.
    if reactor.join().ok().is_none() {
        println!("Unable to join into the live-update.");
        std::process::exit(4);
    }
}
```

Example output:

```
Spotify Client (Version 1.0.42.151.g19de0aa6)
Now playing: Tim Minchin - White Wine In The Sun
Volume: 100%
Now playing: Tim Minchin - Encore
Volume: 50%
Volume: 76%
Now playing: Tim Minchin - Ready For This ?
Volume: 100%
```

## F.A.Q.
**It doesn't connect, what's wrong?**    
Make sure that Spotify is running and the SpotifyWebHelper process is active.

If you can't find SpotifyWebHelper.exe in your process list, you might have disabled it by accident. Here's how you enable it:

- Open Spotify
- Press `Ctrl` + `P` to open the preferences
- Scroll down and click 'Show advanced settings'
- In the `Startup and Window Behaviour` section,   
  enable `Allow Spotify to be opened from the web`.

You might wanna restart Spotify after doing that.   

> **Update**: I'm not sure if this option is still exposed nowadays. Spotify 1.1.95 (2022) on macOS doesn't seem to have this anymore, and I'm not sure if Spotify still exposes the local API at all. If it doesn't, this library is pretty much useless. If you know whether this still works, please open an issue and let me know!

[crates-url]: https://crates.io/crates/spotify
[docs-url]: https://docs.rs/spotify
