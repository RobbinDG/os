#![no_std]

use core::{
    arch::naked_asm,
    marker::Sized,
    ops::{Index, IndexMut},
    panic::PanicInfo,
    ptr, slice,
};

use common::SysCall;

use crate::{
    decimal_printable::{DecimalDigits, DecimalPrintable},
    hex_printable::HexPrintable,
};

pub mod decimal_printable;
pub mod hex_printable;
pub mod static_str;

pub enum KernelError {
    NotReady,
    OutOfBounds,
    Busy,
    OutOfMemory,
}

#[panic_handler]
pub fn kernel_panic(_: &PanicInfo) -> ! {
    loop {}
}

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

pub struct Vec<T: Sized> {
    addr: *mut T,
    size: usize,
}

impl<T: Sized> Vec<T> {
    pub fn malloc(size: usize) -> Result<Self, KernelError> {
        let addr = syscall_mmap::<T>(size);
        if addr.is_null() {
            return Err(KernelError::OutOfMemory);
        }
        Ok(Self { addr, size })
    }
}

impl<T: Sized> Drop for Vec<T> {
    fn drop(&mut self) {
        syscall_munmap(self.addr, self.size);
    }
}

impl<T: Sized> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size {
            panic!()
        }
        unsafe { &*self.addr.add(index) }
    }
}

impl<T: Sized> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size {
            panic!()
        }
        unsafe { &mut *self.addr.add(index) }
    }
}

impl<T: Sized> Vec<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.addr as *const T, self.size) }
    }
}

#[inline(never)] // TMP
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

pub unsafe fn print_decimal<T: DecimalPrintable + DecimalDigits>(n: T) {
    match n.as_decimal() {
        Ok(dec) => print_ascii(dec.as_slice()),
        Err(_) => return,
    }
}

pub unsafe fn print_hex<T: HexPrintable>(n: T) {
    match n.as_hex() {
        Ok(hex) => print_ascii(hex.as_slice()),
        Err(_) => return,
    }
}
