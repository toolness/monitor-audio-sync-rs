use std::ptr::null_mut;
use winapi::shared::ntdef::HRESULT;
use winapi::um::winbase::{
    FormatMessageA,
    FORMAT_MESSAGE_FROM_SYSTEM
};
use winapi::shared::winerror::S_OK;

fn panic_with_winapi_error_desc(base_msg: &'static str, hresult: HRESULT) {
    let mut buf: [u8; 1024] = [0; 1024];
    let result = unsafe {
        FormatMessageA(
            FORMAT_MESSAGE_FROM_SYSTEM,
            null_mut(),
            hresult as u32,
            0,
            buf.as_mut_ptr() as *mut i8,
            buf.len() as u32,
            null_mut()
        )
    };
    if result == 0 {
        panic!("Unknown error {}: {:x}!", base_msg, hresult);
    }
    let strlen = result as usize;
    let s = std::str::from_utf8(&buf[0..strlen]).unwrap();
    println!("Error {}: {}", base_msg, s);
}

pub fn validate_hresult(base_msg: &'static str, hresult: HRESULT) {
    if hresult != S_OK {
        panic_with_winapi_error_desc(base_msg, hresult);
    }
}
