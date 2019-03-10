use std::ptr::null_mut;
use winapi::um::winuser::{GetMessageA, SetTimer, WM_CLOSE, WM_TIMER, MSG};

pub enum WaitResult {
    Exit,
    Timer
}

pub struct TimerEventLoop {
    msg: MSG
}

impl TimerEventLoop {
    pub fn new(interval_ms: u32) -> Self {
        let timer_result = unsafe { SetTimer(null_mut(), 0, interval_ms, None) };

        if timer_result == 0 {
            panic!("SetTimer() failed!");
        }

        TimerEventLoop { msg: super::windows_util::create_blank_msg() }
    }

    pub fn wait(&mut self) -> WaitResult {
        loop {
            let result = unsafe { GetMessageA(&mut self.msg, null_mut(), 0, 0) };
            if result == 0 {
                // WM_QUIT was received.
                return WaitResult::Exit;
            } else if result == -1 {
                // Some kind of error occurred, just exit.
                return WaitResult::Exit;
            } else {
                match self.msg.message {
                    WM_CLOSE => {
                        // This could be sent by the `taskkill` program.
                        println!("WM_CLOSE received, exiting gracefully.");
                        return WaitResult::Exit;
                    },
                    WM_TIMER => {
                        return WaitResult::Timer;
                    },
                    _ => {
                        // Unknown message, just loop again.
                    }
                }
            }
        }
    }
}
