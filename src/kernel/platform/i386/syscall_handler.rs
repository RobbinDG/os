use crate::kernel::{platform::i386::interrupt::data::InterruptHandlerData, syscalls::SysCall};

#[inline(never)]
pub unsafe fn syscall(data: &mut InterruptHandlerData) -> usize {
    unsafe { SysCall::transmute_u32(data.reg.eax).call(data) }
}