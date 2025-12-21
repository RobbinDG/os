#![no_std] // don’t use the Rust standard library
#![no_main]
#![feature(lang_items, core_intrinsics, rustc_private)]
#![allow(internal_features)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![deny(unsafe_op_in_unsafe_fn)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod decimal_printable;
mod dyn_array;
mod hex_printable;
mod kernel;
mod printer;
mod programs;
mod shell;
mod static_str;
mod sys_event;
mod util;
mod vga;

use core::arch::asm;

use crate::{kernel::{isr::empty_event_buffer, kernel::KernelAcc}, printer::VGATextWriter, shell::Shell};

static KERNEL: KernelAcc = KernelAcc::new();
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

#[unsafe(no_mangle)] // turns off name mangling so we can easily link to it later.
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        KERNEL.init();
        if let Ok(kernel) = KERNEL.get() {
            let mut vga = kernel.vga_driver().lock();
            if let Some(tty) = VGATextWriter::get_instance(&mut vga) {
                let mut shell = Shell::new(tty);
                loop {
                    asm!("hlt");
                    let event_buf = empty_event_buffer();
                    for i in 0..event_buf.len() {
                        if let Some(Some(event)) = event_buf.get(i) {
                            match event {
                                sys_event::SysEvent::Keyboard => {
                                    if let Some(key) =
                                        kernel.keyboard_driver().lock().keyboard_interrupt_handler()
                                    {
                                        shell.handle_key(key);
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        loop {}
    }
}
