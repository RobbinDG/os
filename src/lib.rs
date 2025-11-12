#![no_std] // don’t use the Rust standard library
#![no_main] // you’ll provide your own entry point (optional)

mod idt;
mod interrupt_handlers;
mod isr;
mod keyboard_driver;
mod pic;
mod ports;
mod printer;
mod ps2;
mod sys_event;
mod util;
mod vga;

use core::arch::asm;

use crate::{
    isr::{empty_event_buffer, set_isr},
    keyboard_driver::KeyboardDriver,
    printer::TTY,
    ps2::{identity_devices, tmp},
};
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

#[unsafe(no_mangle)] // turns off name mangling so we can easily link to it later.
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        if let Some(mut tty) = TTY::get_instance() {
            tty.clear();
            tty.println_ascii("This is kernel_main.rs".as_bytes());
            set_isr();
            asm!("sti"); // Sets the enable interrupt flag.
            tmp();
        }
        let mut keyboard_drv = match KeyboardDriver::initialise() {
            Ok(drv) => drv,
            Err(_) => {
                if let Some(mut tty) = TTY::get_instance() {
                    tty.println_ascii("Couldn't load keyboard driver.".as_bytes());
                }
                loop {}
            }
        };
        loop {
            asm!("hlt");
            let event_buf = empty_event_buffer();
            for i in 0..event_buf.len() {
                if let Some(Some(event)) = event_buf.get(i) {
                    match event {
                        sys_event::SysEvent::Keyboard => keyboard_drv.keyboard_interrupt_handler(),
                    }
                } else {
                    break;
                }
            }
        }
    }
}
