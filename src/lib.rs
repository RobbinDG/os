#![no_std]  // don’t use the Rust standard library
#![no_main] // you’ll provide your own entry point (optional)

mod printer;
mod ports;
mod vga;
mod util;
mod isr;
mod idt;

use core::arch::asm;

use crate::{ isr::set_isr, printer::TTY};
/*
use core::ptr;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        ptr::copy_nonoverlapping(src, dest, n);
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        ptr::write_bytes(s, c as u8, n);
        s
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        ptr::copy(src, dest, n);
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            let a = *s1.add(i);
            let b = *s2.add(i);
            if a != b {
                return a as i32 - b as i32;
            }
        }
        0
    }
}
    */

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]  // turns off name mangling so we can easily link to it later.
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        set_isr();
        asm!("int 2");
    }
    loop {}
}
