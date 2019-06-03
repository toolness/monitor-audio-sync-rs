// The display that represents the alternative to your normal
// display, e.g. the TV that you sometimes want to use.
//
// If you don't know what to set this to, you can try running
// `cargo run`, change your primary monitor, and observe the
// stdout of the program.
pub const SPECIAL_DISPLAY: &'static [u8] = b"\\\\.\\DISPLAY3\0";

// The audio device that you normally play audio from. You can see
// a list of your audio devices by running `cargo run` and observing
// the stdout of the program.
pub const DEFAULT_AUDIO: &str = "Speakers (Realtek High Definition Audio)";

// The audio device you want to automatically switch to when
// SPECIAL_DISPLAY becomes the your primary monitor.
pub const SPECIAL_AUDIO: &str = "55P607 (NVIDIA High Definition Audio)";

// How frequently to check whether we need to change the current
// primary audio device, in milliseconds.
pub const INTERVAL_MS: u32 = 2000;
