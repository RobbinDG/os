use crate::{platform::i386::interrupt::data::InterruptHandlerData, syscalls::call};
use common::SysCall;

#[inline(never)]
pub unsafe fn syscall(data: &mut InterruptHandlerData) -> usize {
    unsafe { call(&SysCall::transmute_u32(data.reg.eax), data) }
}