use crate::{isr::Registers, sys_event::SysEvent};

pub unsafe fn keyboard_handler(_regs: Registers) -> Option<SysEvent> {
    return Some(SysEvent::Keyboard);
}

fn letter_from_scan_code(scan_code: u8) -> &'static str {
    match scan_code {
        0x10 => "q",
        _ => "X",
    }
}
