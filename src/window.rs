use winapi::shared::windef::HWND;


pub fn find_window(class_name: &[u8], title: &[u8]) -> Option<HWND> {
    use winapi::um::winuser::FindWindowA;
    use std::ffi::CStr;

    let class_name_cstr = CStr::from_bytes_with_nul(class_name).unwrap();
    let title_cstr = CStr::from_bytes_with_nul(title).unwrap();

    let hwnd = unsafe { FindWindowA(class_name_cstr.as_ptr(), title_cstr.as_ptr()) };

    if hwnd.is_null() {
        None
    } else {
        Some(hwnd)
    }
}
