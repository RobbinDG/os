use core::{arch::naked_asm, ptr};

use crate::kernel::syscalls::SysCall;

#[inline(never)]
pub fn syscall_write<T: Sized>(/* fd: usize, */ buf: &[T], count: usize) -> usize {
    let buf_ptr = buf.as_ptr();
    unsafe {
        run_syscall(
            SysCall::Write as usize,
            buf_ptr as usize,
            count * core::mem::size_of::<T>(),
        )
    }
}

#[inline(never)]
pub fn syscall_read<T: Sized>(buf: &mut [T], count: usize) -> usize {
    let buf_ptr = buf.as_ptr();
    unsafe {
        run_syscall(
            SysCall::Read as usize,
            buf_ptr as usize,
            count * core::mem::size_of::<T>(),
        )
    }
}

#[inline(never)]
pub fn syscall_mmap<T: Sized>(count: usize) -> *mut T {
    let res = unsafe { run_syscall(SysCall::MMap as usize, count * core::mem::size_of::<T>(), 0) };
    if res > 0 {
        res as *mut T
    } else {
        ptr::null_mut::<T>()
    }
}

#[inline(never)]
pub fn syscall_munmap<T: Sized>(addr: *mut T, count: usize) {
    unsafe {
        run_syscall(
            SysCall::MUnmap as usize,
            addr as usize,
            count * core::mem::size_of::<T>(),
        )
    };
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

pub struct MemBlock<T: Sized> {
    addr: *mut T,
    size: usize,
}

impl<T: Sized> MemBlock<T> {
    pub fn malloc(size: usize) -> Option<Self> {
        let addr = syscall_mmap::<T>(size);
        if addr.is_null() {
            return None;
        }
        Some(Self { addr, size })
    }
}

impl<T: Sized> Drop for MemBlock<T> {
    fn drop(&mut self) {
        syscall_munmap(self.addr, self.size);
    }
}

pub fn print_ascii(s: &[u8]) {
    // TODO handle output to ensure safety
    syscall_write(s, s.len());
}

pub fn println_ascii(s: &[u8]) {
    // TODO handle output to ensure safety
    // TODO combine into 1 syscall for efficiency
    syscall_write(s, s.len());
    syscall_write(&[b'\n'], 1);
}
