use crate::{kernel::isr::InterruptHandlerData, sys_event::SysEvent};

pub unsafe fn keyboard_handler(_regs: InterruptHandlerData) -> Option<SysEvent> {
    return Some(SysEvent::Keyboard);
}
