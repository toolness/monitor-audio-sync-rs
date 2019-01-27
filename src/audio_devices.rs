use std::ptr::null_mut;
use winapi::um::mmdeviceapi::{IMMDeviceEnumerator, CLSID_MMDeviceEnumerator};
use winapi::um::combaseapi::{CoCreateInstance, CLSCTX_ALL};
use winapi::Interface;

pub fn get_audio_devices() {
    let mut enumerator_ptr = null_mut();

    super::hresult::validate_hresult("obtaining MMDeviceEnumerator", unsafe {
        CoCreateInstance(
            &CLSID_MMDeviceEnumerator,
            null_mut(),
            CLSCTX_ALL,
            &IMMDeviceEnumerator::uuidof(),
            &mut enumerator_ptr
        )
    });

    let enumerator = enumerator_ptr as *mut IMMDeviceEnumerator;

    unsafe { (*enumerator).Release(); }
}
