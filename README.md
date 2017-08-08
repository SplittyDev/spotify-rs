# Spotify-rs
[![Travis CI](https://img.shields.io/travis/SplittyDev/spotify-rs/master.svg?style=flat-square)][travis-url]
[![Crates.io](https://img.shields.io/crates/l/spotify.svg?style=flat-square)][crates-url]
[![Crates.io](https://img.shields.io/crates/v/spotify.svg?style=flat-square)][crates-url]

[Documentation][docs-url]

Easy to use Spotify Local API abstraction library.

NEW in version 0.7.0: Automatically fixes broken or incomplete track URIs.

## What can I do with it
Spotify-rs provides an easy-to-use abstraction over the Spotify Local API.   
It is made for communicating with the local Spotify client in a straightforward way.

You can easily retrieve the currently playing track, the artist who made it,   
the album it's from, the version of the Spotify Client, whether the Spotify Client   
is online or offline, etc.

Spotify-rs supports asynchronous polling, so now   
you can just register a callback, sit back and watch your application deliver   
live information about the track you're currently playing, the current volume,   
or, really, anything else supported by the Spotify Local API.

## Examples
The following is a minimal example to show you what's possible with spotify-rs.   
Please note that this example code lacks error checking, it's really only a quick demonstration.

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

Of course I also have a complete example for you,   
with proper error checking and long polling! :P

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
[docs-url]: https://docs.rs/spotify
