use crate::{isr::Registers, sys_event::SysEvent};

pub unsafe fn null_handler(_regs: Registers) -> Option<SysEvent> { None }
