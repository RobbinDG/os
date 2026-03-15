use crate::kernel::platform::i386::interrupt::data::InterruptHandlerData;

#[inline(never)]
pub unsafe fn syscall(data: &mut InterruptHandlerData) {
    data.reg.eax = 0x420;
}