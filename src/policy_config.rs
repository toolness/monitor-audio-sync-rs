use std::ptr::null_mut;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::shared::ntdef::{HRESULT, PCWSTR};
use winapi::shared::mmreg::WAVEFORMATEX;
use winapi::um::combaseapi::{CoCreateInstance, CLSCTX_ALL};
use winapi::Interface;

// This is taken from:
// https://github.com/DanStevens/AudioEndPointController/blob/master/EndPointController/PolicyConfig.h

// For more details, see:
// http://www.daveamenta.com/2011-05/programmatically-or-command-line-change-the-default-sound-playback-device-in-windows-7/

RIDL!{#[uuid(0xf8679f50, 0x850a, 0x41cf, 0x9c, 0x72, 0x43, 0x0f, 0x29, 0x02, 0x90, 0xc8)]
interface IPolicyConfig(IPolicyConfigVtbl): IUnknown(IUnknownVtbl) {
    fn GetMixFormat(
        a: PCWSTR,
        b: *mut (*mut WAVEFORMATEX),
    ) -> HRESULT,

    // TODO ADD MORE
}}

DEFINE_GUID!{CPolicyConfigClient, 0x870af99c, 0x171d, 0x4f9e, 0xaf, 0x0d, 0xe6, 0x3d, 0xf4, 0x0c, 0x2b, 0xc9}

pub fn get_policy_config_client() {
    let mut policy_config = null_mut();

    super::hresult::validate_hresult("obtaining CPolicyConfigClient", unsafe {
        CoCreateInstance(
            &CPolicyConfigClient,
            null_mut(),
            CLSCTX_ALL,
            &IPolicyConfig::uuidof(),
            &mut policy_config
        )
    });
}
