#![no_std]
#![feature(core_intrinsics)]

pub enum SysCall {
    Unknown = 0,
    Write = 1,
    Read = 2,
    MMap = 3,
    MUnmap = 4,
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
