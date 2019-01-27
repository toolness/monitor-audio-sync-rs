use winapi::um::winuser::{
    MonitorFromPoint,
    GetMonitorInfoA,
    MONITOR_DEFAULTTOPRIMARY,
    MONITORINFOEXA,
    CCHDEVICENAME
};
use winapi::shared::windef::POINT;


pub fn is_display_primary_monitor(display: &[u8]) -> bool {
    let origin = POINT { x: 65535, y: 65535 };
    let hmonitor = unsafe { MonitorFromPoint(origin, MONITOR_DEFAULTTOPRIMARY) };
    let mut info: MONITORINFOEXA = unsafe { std::mem::zeroed() };
    info.cbSize = std::mem::size_of::<MONITORINFOEXA>() as u32;
    let result = unsafe { GetMonitorInfoA(hmonitor, std::mem::transmute(&mut info)) };
    if result == 0 {
        panic!("GetMonitorInfoA() failed!");
    }

    let arr = unsafe { &*(&info.szDevice as *const [i8; CCHDEVICENAME] as *const [u8; CCHDEVICENAME]) };

    &arr[0..display.len()] == display
}
