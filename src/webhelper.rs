#![allow(dead_code)]

// use connector::SpotifyConnector;
use windows_process::WindowsProcess;

const WEBHELPER_PROCESS: &'static str = "SpotifyWebHelper.exe";

/// The `SpotifyWebHelper` struct.
pub struct SpotifyWebHelper;

/// Implements `SpotifyWebHelper`.
impl SpotifyWebHelper {
    fn grab_process() -> Option<WindowsProcess> {
        WindowsProcess::find_by_name(WEBHELPER_PROCESS.into())
    }
    /// Checks whether the process is active.
    pub fn is_alive() -> bool {
        SpotifyWebHelper::grab_process().is_some()
    }
}