#![cfg_attr(debug_assertions, windows_subsystem = "console")]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use(RIDL, DEFINE_GUID, DEFINE_PROPERTYKEY)]
extern crate winapi;

mod monitor;
mod policy_config;
mod hresult;
mod audio_devices;
mod lpwstr;
mod windows_util;
mod timer_event_loop;
mod config;

fn main() {
    use std::ptr::null_mut;
    use winapi::um::objbase::CoInitialize;
    use self::timer_event_loop::{TimerEventLoop, WaitResult};

    hresult::validate_hresult("calling CoInitialize()", unsafe {
        CoInitialize(null_mut())
    });

    let mut event_loop = TimerEventLoop::new(config::INTERVAL_MS);
    let mut current_primary_monitor_is_special = None;

    loop {
        match event_loop.wait() {
            WaitResult::Timer => {
                let is_special = monitor::is_display_primary_monitor(config::SPECIAL_DISPLAY);

                if current_primary_monitor_is_special != Some(is_special) {
                    let name = if is_special { config::SPECIAL_AUDIO } else { config::DEFAULT_AUDIO };
                    if let Some(id) = audio_devices::get_audio_device_id(name) {
                        println!("Changing audio device to {}.", name);
                        policy_config::set_default_audio_playback_device(id.get_ptr());
                    }
                    current_primary_monitor_is_special = Some(is_special);
                }
            },
            WaitResult::Exit => {
                break;
            }
        }
    }
}
