use crate::{programs::lib::syscall_read, shell::Shell};

/// The main function to-be for the shell program.
#[inline(never)]
pub fn run_shell() {
    let mut shell = unsafe { Shell::new() };

    let mut buf = [0u8; 16];
    loop {
        let bytes_read = syscall_read(&mut buf, 1);
        if bytes_read > 0 {
            unsafe { shell.handle_key(buf[0]) };
        }
    }
}
