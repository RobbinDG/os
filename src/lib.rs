#![no_std] // don’t use the Rust standard library
#![no_main]
#![deny(clippy::unwrap_used, clippy::expect_used)]

mod dyn_array;
mod hex_printable;
mod idt;
mod interrupt_handlers;
mod isr;
mod kernel;
mod keyboard_driver;
mod pic;
mod ports;
mod printer;
mod programs;
mod ps2;
mod shell;
mod static_str;
mod sys_event;
mod util;
mod vga;

use core::arch::asm;

use crate::{
    isr::{empty_event_buffer, set_isr},
    kernel::kernel::KernelAcc,
    keyboard_driver::KeyboardDriver,
    printer::VGAText,
    ps2::tmp,
    shell::Shell,
};

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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
/// Detects the low memory size and stores it in this object.
/// This implementation is *incredibly* hacky and does not account for errors.
/// Reason 1: The address is hard coded in boot_sect.asm so we know where to read the value
/// Reason 2: The BIOS call can only be made from real mode, so we get it 
///     in the boot sector and put it away for a bit.
/// Reason 3: The address of the stored value is in the boot sector, so we are essentially
///     overwriting it if ever it would get filled up.
/// Reason 4: The interrupt 0x12 comes with an error bit on the carry flag. We don't store 
///     it right now.
/// Reason 5: This doesn't really fix anything, if we want to expand, we need to do all this
///     trickery again.
/// 
/// The solution is veritual 16 bit mode. That way, we can read it during execution.
unsafe fn detect_low_mem() -> u16 {
    let addr = 508 as *const u16;
    unsafe { *addr }
}

#[unsafe(no_mangle)] // turns off name mangling so we can easily link to it later.
pub extern "C" fn kernel_main() -> ! {
    KERNEL.init(unsafe { detect_low_mem() } );
    unsafe {
        if let Some(mut tty) = VGAText::get_instance() {
            tty.clear();
            tty.println_ascii("This is kernel_main.rs".as_bytes());
            set_isr();
            asm!("sti"); // Sets the enable interrupt flag.
            tmp();
        }
        let mut keyboard_drv = match KeyboardDriver::initialise() {
            Ok(drv) => drv,
            Err(_) => {
                if let Some(mut tty) = VGAText::get_instance() {
                    tty.println_ascii("Couldn't load keyboard driver.".as_bytes());
                }
                loop {}
            }
        };
        if let Some(tty) = VGAText::get_instance() {
            let mut shell = Shell::new(tty);
            loop {
                asm!("hlt");
                let event_buf = empty_event_buffer();
                for i in 0..event_buf.len() {
                    if let Some(Some(event)) = event_buf.get(i) {
                        match event {
                            sys_event::SysEvent::Keyboard => {
                                if let Some(key) = keyboard_drv.keyboard_interrupt_handler() {
                                    shell.handle_key(key);
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            loop {}
        }
    }
}
