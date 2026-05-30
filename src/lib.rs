#![no_std] // don’t use the Rust standard library
#![no_main]
#![feature(
    lang_items,
    core_intrinsics,
    rustc_private,
    thread_local,
    unsafe_cell_access
)]
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
mod programs;
mod shell;
mod static_str;
mod sys_event;
mod util;
mod vga;

use crate::{
    kernel::{
        global::Global, kernel::init_kernel, platform::i386::context_switch::ProcessCtrlBlock,
        scheduler::Scheduler,
    },
    programs::{lib::syscall_write, run_shell::run_shell},
};

static SCHEDULER: Global<Scheduler> = unsafe { Global::new(Scheduler::new()) };

#[inline(never)]
pub unsafe extern "C" fn scheduler_entrypoint() -> ! {
    loop {
        let mut process = ProcessCtrlBlock::new_process(0x7FFFF, run_shell);
        unsafe {
            SCHEDULER.with::<()>(|s| {
                s.run_process(&mut process);
            })
        };
    }
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn switch_to_scheduler() -> ! {
//     if let Ok(kernel) = KERNEL.get() {
//         let cur = unsafe { kernel.scheduler.current_process() };
//         let new = kernel.scheduler.scheduler_process();
//         unsafe { switch_context(cur, new) }
//     } else {
//         panic!()
//     }
// }
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
#[inline(never)]
fn sample_process() {
    let buf = [b'A'];
    syscall_write(&buf, buf.len());
}

#[unsafe(no_mangle)] // turns off name mangling so we can easily link to it later.
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        init_kernel();

        scheduler_entrypoint();
    }
}
