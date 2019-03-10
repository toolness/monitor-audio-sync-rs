// Uncomment the following line to make it possible to
// run the program in console mode.
#![windows_subsystem = "windows"]

#[macro_use(RIDL, DEFINE_GUID, DEFINE_PROPERTYKEY)]
extern crate winapi;

mod monitor;
mod policy_config;
mod hresult;
mod audio_devices;
mod lpwstr;

const SPECIAL_DISPLAY: &'static [u8] = b"\\\\.\\DISPLAY3\0";

const DEFAULT_AUDIO: &str = "Speakers (Realtek High Definition Audio)";

const SPECIAL_AUDIO: &str = "55P607 (NVIDIA High Definition Audio)";

fn main() {
    use winapi::um::objbase::CoInitialize;
    use std::ptr::null_mut;
    use std::{thread, time};

    hresult::validate_hresult("calling CoInitialize()", unsafe {
        CoInitialize(null_mut())
    });

    let interval = time::Duration::from_secs(2);
    let mut current_primary_monitor_is_special = None;

    loop {
        let is_special = monitor::is_display_primary_monitor(SPECIAL_DISPLAY);

        if current_primary_monitor_is_special != Some(is_special) {
            let name = if is_special { SPECIAL_AUDIO } else { DEFAULT_AUDIO };
            if let Some(id) = audio_devices::get_audio_device_id(name) {
                println!("Changing audio device to {}.", name);
                policy_config::set_default_audio_playback_device(id.get_ptr());
            }
            current_primary_monitor_is_special = Some(is_special);
        }

        thread::sleep(interval);
    }
}
