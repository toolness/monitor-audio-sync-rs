use std::ptr::null_mut;
use winapi::um::mmdeviceapi::{
    IMMDevice,
    IMMDeviceEnumerator,
    CLSID_MMDeviceEnumerator,
    IMMDeviceCollection,
    eRender,
    DEVICE_STATE_ACTIVE
};
use winapi::shared::ntdef::LPWSTR;
use winapi::um::coml2api::STGM_READ;
use winapi::um::propsys::IPropertyStore;
use winapi::um::combaseapi::{CoCreateInstance, CLSCTX_ALL, FreePropVariantArray};
use winapi::um::propidl::PROPVARIANT;
use winapi::shared::wtypes::{VT_LPWSTR, VARTYPE};
use winapi::shared::minwindef::WORD;
use winapi::Interface;

use super::hresult::validate_hresult;

DEFINE_PROPERTYKEY!(PKEY_Device_FriendlyName, 0xa45c254e, 0xdf1c, 0x4efd, 0x80, 0x20, 0x67, 0xd1, 0x46, 0xa8, 0x50, 0xe0, 14); // DEVPROP_TYPE_STRING

#[repr(C)]
pub struct PROPVARIANT_LPWSTR {
    vt: VARTYPE,
    w_reserved1: WORD,
    w_reserved2: WORD,
    w_reserved3: WORD,
    pwsz_val: LPWSTR
}

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

    for i in 0..count {
        let mut device: *mut IMMDevice = null_mut();

        validate_hresult("calling Item()", unsafe { (*collection).Item(i, &mut device) });

        let mut propstore: *mut IPropertyStore = null_mut();

        validate_hresult("calling OpenPropertyStore()", unsafe {
            (*device).OpenPropertyStore(STGM_READ, &mut propstore)
        });

        let mut variant: PROPVARIANT = unsafe { std::mem::zeroed() };

        validate_hresult("calling GetValue()", unsafe {
            (*propstore).GetValue(&PKEY_Device_FriendlyName, &mut variant)
        });

        if variant.vt != VT_LPWSTR as u16 {
            panic!("Expected variant type to be VT_LPWSTR!");
        }

        let variant_lpwstr: &PROPVARIANT_LPWSTR = unsafe { std::mem::transmute(&variant) };

        let friendly_name = unsafe { super::lpwstr::lpwstr_to_string(variant_lpwstr.pwsz_val) };

        println!("Friendly name is {}.", friendly_name);

        let mut id_lpwstr: LPWSTR = null_mut();

        validate_hresult("calling GetId()", unsafe {
            (*device).GetId(&mut id_lpwstr)
        });

        let id = super::lpwstr::TaskAllocatedLpwstr::new(id_lpwstr);

        println!("  Its ID is {}.", unsafe { id.to_string() });

        validate_hresult("calling FreePropVariantArray()", unsafe {
            FreePropVariantArray(1, &mut variant)
        });

        unsafe { (*propstore).Release(); }

        unsafe { (*device).Release(); }
    }

    unsafe { (*collection).Release(); }

    unsafe { (*enumerator).Release(); }
}
