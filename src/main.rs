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
mod windows_util;

const SPECIAL_DISPLAY: &'static [u8] = b"\\\\.\\DISPLAY3\0";

const DEFAULT_AUDIO: &str = "Speakers (Realtek High Definition Audio)";

const SPECIAL_AUDIO: &str = "55P607 (NVIDIA High Definition Audio)";

const INTERVAL_MS: u32 = 2000;

fn main() {
    use std::ptr::null_mut;
    use winapi::um::objbase::CoInitialize;
    use winapi::um::winuser::{GetMessageA, SetTimer, WM_CLOSE, WM_TIMER};

    hresult::validate_hresult("calling CoInitialize()", unsafe {
        CoInitialize(null_mut())
    });

    let mut current_primary_monitor_is_special = None;

    let timer_result = unsafe { SetTimer(null_mut(), 0, INTERVAL_MS, None) };

    if timer_result == 0 {
        panic!("SetTimer() failed!");
    }

    let mut msg = windows_util::create_blank_msg();

    loop {
        let result = unsafe { GetMessageA(&mut msg, null_mut(), 0, 0) };
        if result == 0 {
            // WM_QUIT was received.
            break;
        } else if result == -1 {
            // Some kind of error occurred, just exit.
            break;
        } else {
            match msg.message {
                WM_CLOSE => {
                    // This could be sent by the `taskkill` program.
                    println!("WM_CLOSE received, exiting gracefully.");
                    break;
                },
                WM_TIMER => {
                    let is_special = monitor::is_display_primary_monitor(SPECIAL_DISPLAY);

                    if current_primary_monitor_is_special != Some(is_special) {
                        let name = if is_special { SPECIAL_AUDIO } else { DEFAULT_AUDIO };
                        if let Some(id) = audio_devices::get_audio_device_id(name) {
                            println!("Changing audio device to {}.", name);
                            policy_config::set_default_audio_playback_device(id.get_ptr());
                        }
                        current_primary_monitor_is_special = Some(is_special);
                    }
                },
                _ => {
                }
            }
        }
    }
}
