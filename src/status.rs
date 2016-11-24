use json::JsonValue;

/// A Spotify status.
pub struct SpotifyStatus {
    /// The volume.
    /// Valid values are [0.0...1.0].
    pub volume: f32,
    /// Whether the client is online.
    pub online: bool,
    /// The protocol version.
    pub version: i32,
    /// Whether the client is running.
    pub running: bool,
    /// Whether a track is currently playing.
    pub playing: bool,
    /// Whether shuffle mode is activated.
    pub shuffle: bool,
    /// The server time as a unix timestamp.
    pub server_time: i64,
    /// Whether playing a track is enabled.
    pub play_enabled: bool,
    /// Whether playing the previous track is enabled.
    pub prev_enabled: bool,
    /// Whether playing the next track is enabled.
    pub next_enabled: bool,
    /// The client version.
    pub client_version: String,
    /// The current playing position.
    pub playing_position: f32,
    /// The Open Graph state.
    pub open_graph_state: OpenGraphState,
    /// The currently playing track.
    pub track: Option<Track>,
}

/// A Spotify Open Graph state.
pub struct OpenGraphState {
    /// Whether the current session is private.
    pub private_session: bool,
    /// Whether posting is disabled.
    pub posting_disabled: bool,
}

/// A Spotify track.
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
pub struct Resource {
    /// The internal resource uri.
    pub uri: String,
    /// The name.
    pub name: String,
    /// The location.
    pub location: ResourceLocation,
}

/// A Spotify resource location.
pub struct ResourceLocation {
    /// The online resource url.
    pub og: String,
}

/// Transforms a JSON value into an owned String.
fn get_json_str(json: &JsonValue) -> String {
    match json.as_str() {
        Some(val) => val.to_owned(),
        None => String::default(),
    }
}

/// Implements `From<JsonValue>` for `SpotifyStatus`.
impl From<JsonValue> for SpotifyStatus {
    fn from(json: JsonValue) -> SpotifyStatus {
        SpotifyStatus {
            volume: json["volume"].as_f32().unwrap_or(0f32),
            online: json["online"] == true,
            version: json["version"].as_i32().unwrap_or(0i32),
            running: json["running"] == true,
            playing: json["playing"] == true,
            shuffle: json["shuffle"] == true,
            server_time: json["server_time"].as_i64().unwrap_or(0i64),
            play_enabled: json["play_enabled"] == true,
            prev_enabled: json["prev_enabled"] == true,
            next_enabled: json["next_enabled"] == true,
            client_version: get_json_str(&json["client_version"]),
            playing_position: json["playing_position"].as_f32().unwrap_or(0f32),
            open_graph_state: OpenGraphState::from(&json["open_graph_state"]),
            track: match json["track"].is_null() {
                true => None,
                false => Some(Track::from(&json["track"])),
            },
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
            length: json["length"].as_i32().unwrap_or(0i32),
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