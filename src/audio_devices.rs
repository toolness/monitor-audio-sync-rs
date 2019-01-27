use std::ptr::null_mut;
use winapi::um::mmdeviceapi::{
    IMMDeviceEnumerator,
    CLSID_MMDeviceEnumerator,
    IMMDeviceCollection,
    eRender,
    DEVICE_STATE_ACTIVE
};
use winapi::um::combaseapi::{CoCreateInstance, CLSCTX_ALL};
use winapi::Interface;

use super::hresult::validate_hresult;

pub fn get_audio_devices() {
    let mut enumerator_ptr = null_mut();

    validate_hresult("obtaining MMDeviceEnumerator", unsafe {
        CoCreateInstance(
            &CLSID_MMDeviceEnumerator,
            null_mut(),
            CLSCTX_ALL,
            &IMMDeviceEnumerator::uuidof(),
            &mut enumerator_ptr
        )
    });

    let enumerator = enumerator_ptr as *mut IMMDeviceEnumerator;

    let mut collection: *mut IMMDeviceCollection = null_mut();

    validate_hresult("calling EnumAudioEndpoints()", unsafe {
        (*enumerator).EnumAudioEndpoints(
            eRender,
            DEVICE_STATE_ACTIVE,
            &mut collection
        )
    });

    let mut count: u32 = 0;

    validate_hresult("calling GetCount()", unsafe { (*collection).GetCount(&mut count) });

    println!("You have {} audio devices for playback.", count);

    unsafe { (*enumerator).Release(); }

    unsafe { (*enumerator).Release(); }
}
