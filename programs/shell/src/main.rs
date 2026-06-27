#![no_std]

use crate::shell::Shell;
use stdlib::syscall_read;

mod shell;
mod ps2_cli;

/// The main function to-be for the shell program.
#[inline(never)]
pub fn main() {
    let mut shell = unsafe { Shell::new() };

    let mut buf = [0u8; 16];
    loop {
        let bytes_read = syscall_read(&mut buf, 1);
        if bytes_read > 0 {
            unsafe { shell.handle_key(buf[0]) };
        }
    }
}
