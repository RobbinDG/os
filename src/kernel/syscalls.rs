use crate::{KERNEL, kernel::platform::i386::interrupt::data::InterruptHandlerData};

pub enum SysCall {
    Unknown = 0,
    Write = 1,
    Read = 2,
    _LastDummy,
}

impl SysCall {
    pub unsafe fn transmute_u32(value: u32) -> Self {
        unsafe {
            // We expect to call this often, and correctly. Hint to the compiler
            // that this is the case so it will compile the comparisons favourably.
            if core::intrinsics::unlikely(value > SysCall::_LastDummy as u32) {
                SysCall::Unknown
            } else {
                core::mem::transmute(value as u8)
            }
        }
    }

    #[inline]
    pub unsafe fn call(&self, data: &mut InterruptHandlerData) -> usize {
        unsafe {
            match self {
                SysCall::Unknown => 255,
                SysCall::Write => Self::write(data),
                SysCall::Read => Self::read(data),
                SysCall::_LastDummy => 255,
            }
        }
    }

    unsafe fn write(regs: &mut InterruptHandlerData) -> usize {
        unsafe {
            KERNEL
                .tmp_tty
                .with_unwrap(|tty| tty.write(*(regs.reg.ecx as *const u8) as u8))
        }
    }

    unsafe fn read(regs: &mut InterruptHandlerData) -> usize {
        unsafe {
            KERNEL.tmp_tty.with_unwrap(|tty| {
                tty.read(
                    core::slice::from_raw_parts_mut(regs.reg.ecx as *mut u8, regs.reg.edx as usize),
                    regs.reg.edx as usize,
                )
            })
        }
    }
}
