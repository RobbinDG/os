use crate::sys_event::SysEvent;

pub const EVENT_BUF_SIZE: usize = 8;
pub struct EventBuf {
    pub buf: [Option<SysEvent>; EVENT_BUF_SIZE],
    pub len: usize,
}

pub static mut EVENT_BUF: EventBuf = EventBuf {
    buf: [None; EVENT_BUF_SIZE],
    len: 0,
};

pub unsafe fn empty_event_buffer() -> [Option<SysEvent>; EVENT_BUF_SIZE] {
    unsafe {
        let cp = EVENT_BUF.buf;
        while EVENT_BUF.len > 0 {
            EVENT_BUF.len -= 1;
            EVENT_BUF.buf[EVENT_BUF.len] = None;
        }
        cp
    }
}
