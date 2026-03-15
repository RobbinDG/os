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
    pub unsafe fn call(&self, data: &mut InterruptHandlerData) {
        unsafe {
            match self {
                SysCall::Unknown => {}
                SysCall::Write => Self::write(data),
                SysCall::Read => {}
                SysCall::_LastDummy => {}
            }
        }
    }

    unsafe fn write(regs: &mut InterruptHandlerData) {
        unsafe {
            KERNEL.vga_driver.with(|vga| {
                vga.put_char_raw(*(regs.reg.ecx as *const u8) as u8, 0, 0);
            })
        }
    }
}
