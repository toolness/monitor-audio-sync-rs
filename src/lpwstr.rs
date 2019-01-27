use std::ffi::OsString;
use std::os::windows::prelude::*;
use winapi::shared::ntdef::LPWSTR;
use winapi::ctypes::c_void;
use winapi::um::combaseapi::CoTaskMemFree;

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

#[derive(Debug)]
pub struct TaskAllocatedLpwstr {
    ptr: LPWSTR,
}

impl TaskAllocatedLpwstr {
    pub fn new(ptr: LPWSTR) -> Self {
        TaskAllocatedLpwstr { ptr }
    }

    pub fn get_ptr(&self) -> LPWSTR {
        self.ptr
    }

    pub unsafe fn to_string(&self) -> String {
        lpwstr_to_string(self.ptr)
    }
}

impl Drop for TaskAllocatedLpwstr {
    fn drop(&mut self) {
        unsafe { CoTaskMemFree(self.ptr as *mut c_void) };
    }
}
