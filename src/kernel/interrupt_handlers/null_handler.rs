use crate::{kernel::isr::InterruptHandlerData, sys_event::SysEvent};

pub unsafe fn null_handler(_regs: InterruptHandlerData) -> Option<SysEvent> { None }
