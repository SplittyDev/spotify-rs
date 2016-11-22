#![allow(dead_code)]

use windows_process::WindowsProcess;

/// The `SpotifyWebHelper` struct.
pub struct SpotifyWebHelper;

/// Implements `SpotifyWebHelper`.
impl SpotifyWebHelper {
    fn grab_process(self) -> Option<WindowsProcess> {
        WindowsProcess::find_by_name("SpotifyWebHelper.exe".into())
    }
    /// Checks whether the process is active.
    pub fn is_alive(self) -> bool {
        self.grab_process().is_some()
    }
}