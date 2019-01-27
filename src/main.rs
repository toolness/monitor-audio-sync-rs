mod monitor;
mod window;

const SPECIAL_DISPLAY: &'static [u8] = b"\\\\.\\DISPLAY3\0";

// https://docs.microsoft.com/en-us/windows/desktop/winmsg/about-window-classes
const DIALOG_BOX_WINDOW_CLASS: &'static [u8] = b"#32770\0";

const SOUND_DIALOG_TITLE: &'static [u8] = b"Sound\0";

fn main() {
    println!("is_display_primary_monitor(SPECIAL_DISPLAY) = {}",
             monitor::is_display_primary_monitor(SPECIAL_DISPLAY));

    if let Some(hwnd) = window::find_window(DIALOG_BOX_WINDOW_CLASS, SOUND_DIALOG_TITLE) {
        println!("Found sound window {:?}", hwnd);
    } else {
        // TODO: Run "control mmsys.cpl".
        println!("No sound window found.");
    }
}
