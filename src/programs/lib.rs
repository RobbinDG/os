use core::arch::naked_asm;

use crate::kernel::syscalls::SysCall;

#[inline(never)]
pub fn syscall_write<T: Sized>(/* fd: usize, */ buf: &[T], count: usize) -> usize {
    let buf_ptr = buf.as_ptr();
    unsafe { run_syscall(SysCall::Write as usize, buf_ptr as usize, count) }
}

#[inline(never)]
pub fn syscall_read<T: Sized>(buf: &mut [T], count: usize) -> usize {
    let buf_ptr = buf.as_ptr();
    unsafe { run_syscall(SysCall::Read as usize, buf_ptr as usize, count) }
}

#[unsafe(naked)]
unsafe extern "C" fn run_syscall(function: usize, arg1: usize, arg2: usize) -> usize {
    naked_asm!(
        "mov eax, [esp + 4]",
        "mov ecx, [esp + 8]",
        "mov edx, [esp + 12]",
        "int 0x80",
        "ret", // assume return value is in `eax`
    )
}
