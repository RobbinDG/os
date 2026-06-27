use crate::{platform::i386::interrupt::data::InterruptHandlerData, sys_event::SysEvent};

pub unsafe fn null_handler(_regs: InterruptHandlerData) -> Option<SysEvent> {
    None
}
