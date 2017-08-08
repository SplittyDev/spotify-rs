//! The status module.
//!
//! This module contains methods to turn the JSON responses
//! from the Spotify connector into easy-to-use structures.
//!
//! It also contains some extra abstractions, such as the `SimpleTrack` struct.

use json::JsonValue;
use time::{self, Timespec, Tm};

/// A change in the Spotify status.
pub struct SpotifyStatusChange {
    /// Indicates a change in the volume.
    pub volume: bool,
    /// Indicates a change in the online status.
    pub online: bool,
    /// Indicates a change in the protocol version.
    pub version: bool,
    /// Indicates a change in the running state.
    pub running: bool,
    /// Indicates a change in the playing state.
    pub playing: bool,
    /// Indicates a change in the shuffle mode.
    pub shuffle: bool,
    /// Indicates a change in the server time.
    pub server_time: bool,
    /// Indicates a change in the play enabled state.
    pub play_enabled: bool,
    /// Indicates a change in the prev enabled state.
    pub prev_enabled: bool,
    /// Indicates a change in the next enabled state.
    pub next_enabled: bool,
    /// Indicates a change in the client version.
    pub client_version: bool,
    /// Indicates a change in the playing position.
    pub playing_position: bool,
    /// Indicates a change in the open graph data.
    pub open_graph_state: bool,
    /// Indicates a change in the track.
    pub track: bool,
}

/// A Spotify status.
#[derive(Debug, Clone, PartialEq)]
pub struct SpotifyStatus {
    /// The volume.
    /// Valid values are [0.0...1.0].
    volume: f32,
    /// Whether the client is online.
    online: bool,
    /// The protocol version.
    version: i32,
    /// Whether the client is running.
    running: bool,
    /// Whether a track is currently playing.
    playing: bool,
    /// Whether shuffle mode is activated.
    shuffle: bool,
    /// The server time as a unix timestamp.
    server_time: i64,
    /// Whether playing a track is enabled.
    play_enabled: bool,
    /// Whether playing the previous track is enabled.
    prev_enabled: bool,
    /// Whether playing the next track is enabled.
    next_enabled: bool,
    /// The client version.
    client_version: String,
    /// The current playing position.
    playing_position: f32,
    /// The Open Graph state.
    open_graph_state: OpenGraphState,
    /// The currently playing track.
    track: Track,
}

/// A Spotify Open Graph state.
#[derive(Debug, Clone, PartialEq)]
struct OpenGraphState {
    /// Whether the current session is private.
    private_session: bool,
    /// Whether posting is disabled.
    posting_disabled: bool,
}

/// A Spotify track.
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    /// The track.
    pub track: Resource,
    /// The album.
    pub album: Resource,
    /// The artist.
    pub artist: Resource,
    /// The length in full seconds.
    pub length: i32,
    /// The track type.
    pub track_type: String,
}

/// A Spotify resource.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    /// The internal resource uri.
    pub uri: String,
    /// The name.
    pub name: String,
    /// The location.
    pub location: ResourceLocation,
}

/// A Spotify resource location.
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceLocation {
    /// The online resource url.
    pub og: String,
}

/// A simple track.
/// Provides an abstraction over the more
/// complicated and quite messy `Track` struct.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTrack {
    /// The track name.
    pub name: String,
    /// The album name.
    pub album: String,
    /// The artist name.
    pub artist: String,
}

/// Transforms a JSON value into an owned String.
#[inline]
fn get_json_str(json: &JsonValue) -> String {
    match json.as_str() {
        Some(val) => val.to_owned(),
        None => String::default(),
    }
}

/// Implements `SpotifyStatus`.
impl SpotifyStatus {
    /// Gets an easy-to-work-with abstraction over
    /// the currently playing track, containing only
    /// the names of the track, album and artist.
    pub fn track(&self) -> SimpleTrack {
        SimpleTrack::from(&self.track)
    }
    /// Gets all information regarding the
    /// currently playing track.
    pub fn full_track(&self) -> Track {
        self.track.clone()
    }
    /// Gets the client version.
    pub fn version(&self) -> String {
        self.client_version.clone()
    }
    /// Gets the volume.
    /// Possible values range from `0.0_f32` to `1.0_f32`.
    pub fn volume(&self) -> f32 {
        self.volume
    }
    /// Gets the volume as percentage.
    /// Possible values range from `0.0_f32` to `100.0_f32`.
    pub fn volume_percentage(&self) -> f32 {
        (self.volume * 100_f32).trunc()
    }
    /// Gets the server timestamp.
    pub fn timestamp(&self) -> i64 {
        self.server_time
    }
    /// Gets the local server time.
    pub fn time(&self) -> Tm {
        time::at(Timespec::new(self.server_time, 0))
    }
    /// Gets the coordinated universal server time.
    pub fn time_utc(&self) -> Tm {
        time::at_utc(Timespec::new(self.server_time, 0))
    }
    /// Gets a value indicating whether shuffling is enabled.
    pub fn shuffle_enabled(&self) -> bool {
        self.shuffle
    }
    /// Gets a value indicating whether the client is
    /// currently connected to the Internet.
    pub fn is_online(&self) -> bool {
        self.online
    }
    /// Gets a value indicating whether the current
    /// session is a private session.
    pub fn is_private_session(&self) -> bool {
        self.open_graph_state.private_session
    }
}

/// Implements `SpotifyStatusChange`.
impl SpotifyStatusChange {
    /// Constructs a new `SpotifyStatusChange` with all fields set to true.
    pub fn new_true() -> SpotifyStatusChange {
        SpotifyStatusChange {
            volume: true,
            online: true,
            version: true,
            running: true,
            playing: true,
            shuffle: true,
            server_time: true,
            play_enabled: true,
            prev_enabled: true,
            next_enabled: true,
            client_version: true,
            playing_position: true,
            open_graph_state: true,
            track: true,
        }
    }
}

/// Implements `From<JsonValue>` for `SpotifyStatus`.
impl From<JsonValue> for SpotifyStatus {
    fn from(json: JsonValue) -> SpotifyStatus {
        SpotifyStatus {
            volume: json["volume"].as_f32().unwrap_or(0_f32),
            online: json["online"] == true,
            version: json["version"].as_i32().unwrap_or(0_i32),
            running: json["running"] == true,
            playing: json["playing"] == true,
            shuffle: json["shuffle"] == true,
            server_time: json["server_time"].as_i64().unwrap_or(0_i64),
            play_enabled: json["play_enabled"] == true,
            prev_enabled: json["prev_enabled"] == true,
            next_enabled: json["next_enabled"] == true,
            client_version: get_json_str(&json["client_version"]),
            playing_position: json["playing_position"].as_f32().unwrap_or(0_f32),
            open_graph_state: OpenGraphState::from(&json["open_graph_state"]),
            track: Track::from(&json["track"]),
        }
    }
}

/// Implements `From<&'a JsonValue>` for `OpenGraphState`.
impl<'a> From<&'a JsonValue> for OpenGraphState {
    fn from(json: &'a JsonValue) -> OpenGraphState {
        OpenGraphState {
            private_session: json["private_session"] == true,
            posting_disabled: json["posting_disabled"] == true,
        }
    }
}

/// Implements `From<&'a JsonValue>` for `Track`.
impl<'a> From<&'a JsonValue> for Track {
    fn from(json: &'a JsonValue) -> Track {
        Track {
            track_type: get_json_str(&json["uri"]),
            track: Resource::from(&json["track_resource"]),
            album: Resource::from(&json["album_resource"]),
            artist: Resource::from(&json["artist_resource"]),
            length: json["length"].as_i32().unwrap_or(0_i32),
        }
    }
}

/// Implements `From<&'a JsonValue>` for `Resource`.
impl<'a> From<&'a JsonValue> for Resource {
    fn from(json: &'a JsonValue) -> Resource {
        Resource {
            uri: get_json_str(&json["uri"]),
            name: get_json_str(&json["name"]),
            location: ResourceLocation::from(&json["location"]),
        }
    }
}

/// Implements `From<&'a JsonValue>` for `ResourceLocation`.
impl<'a> From<&'a JsonValue> for ResourceLocation {
    fn from(json: &'a JsonValue) -> ResourceLocation {
        ResourceLocation { og: get_json_str(&json["og"]) }
    }
}

/// Implements `From<Track>` for `SimpleTrack`.
impl<'a> From<&'a Track> for SimpleTrack {
    fn from(track: &'a Track) -> SimpleTrack {
        SimpleTrack {
            name: track.track.name.clone(),
            album: track.album.name.clone(),
            artist: track.artist.name.clone(),
        }
    }
}

/// Implements `From<SpotifyStatus>` for `SimpleTrack`.
impl<'a> From<&'a SpotifyStatus> for SimpleTrack {
    fn from(status: &'a SpotifyStatus) -> SimpleTrack {
        SimpleTrack::from(&status.track)
    }
}

/// Implements `fmt::Display` for `SimpleTrack`.
impl ::std::fmt::Display for SimpleTrack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{} - {}", self.artist, self.name)
    }
}

/// Implements `From<(SpotifyStatus, SpotifyStatus)>` for `SpotifyStatusChange`.
impl From<(SpotifyStatus, SpotifyStatus)> for SpotifyStatusChange {
    fn from(set: (SpotifyStatus, SpotifyStatus)) -> SpotifyStatusChange {
        let curr = set.0;
        let last = set.1;
        macro_rules! status_compare_field {
            ($field:ident) => (curr.$field != last.$field)
        }
        SpotifyStatusChange {
            volume: status_compare_field!(volume),
            online: status_compare_field!(online),
            version: status_compare_field!(version),
            running: status_compare_field!(running),
            playing: status_compare_field!(playing),
            shuffle: status_compare_field!(shuffle),
            server_time: status_compare_field!(server_time),
            play_enabled: status_compare_field!(play_enabled),
            prev_enabled: status_compare_field!(prev_enabled),
            next_enabled: status_compare_field!(next_enabled),
            client_version: status_compare_field!(client_version),
            playing_position: status_compare_field!(playing_position),
            open_graph_state: status_compare_field!(open_graph_state),
            track: status_compare_field!(track),
        }
    }
}