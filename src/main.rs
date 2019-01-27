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

    hresult::validate_hresult("calling CoInitialize()", unsafe {
        CoInitialize(null_mut())
    });

    println!("is_display_primary_monitor(SPECIAL_DISPLAY) = {}",
             monitor::is_display_primary_monitor(SPECIAL_DISPLAY));

    let default_audio_id = audio_devices::get_audio_device_id(DEFAULT_AUDIO);
    let special_audio_id = audio_devices::get_audio_device_id(SPECIAL_AUDIO);

    println!("ID for default audio is {:?}.", default_audio_id);
    println!("ID for special audio is {:?}.", special_audio_id);

    policy_config::set_default_audio_playback_device(default_audio_id.unwrap().get_ptr());
}
