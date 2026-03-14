use crate::kernel::isr::InterruptHandlerData;

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
}

pub struct SysCalls {}

impl SysCalls {
    pub const fn new() -> Self {
        Self {}
    }

    #[inline]
    pub unsafe fn call(data: InterruptHandlerData) {
        unsafe {
            match SysCall::transmute_u32(data.reg.eax) {
                SysCall::Unknown => {}
                SysCall::Write => {}
                SysCall::Read => Self::write(data),
                SysCall::_LastDummy => {}
            }
        }
    }

    pub unsafe fn write(regs: InterruptHandlerData) {}
}
