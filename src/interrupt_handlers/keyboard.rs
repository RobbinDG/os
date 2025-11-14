use crate::{isr::Registers, sys_event::SysEvent};

pub unsafe fn keyboard_handler(_regs: Registers) -> Option<SysEvent> {
    return Some(SysEvent::Keyboard);
}
