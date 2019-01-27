use std::ffi::OsString;
use std::os::windows::prelude::*;
use winapi::shared::ntdef::{LPWSTR};

pub unsafe fn lpwstr_to_string(s: LPWSTR) -> String {
    let mut len = 0;

    loop {
        let ch = *s.offset(len);
        if ch == 0 {
            break;
        }
        len += 1;
    }

    let os_string = OsString::from_wide(std::slice::from_raw_parts(s, len as usize));
    os_string.into_string().unwrap()
}
