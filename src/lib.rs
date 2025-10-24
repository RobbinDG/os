#![no_std]  // don’t use the Rust standard library
#![no_main] // you’ll provide your own entry point (optional)

use crate::printer::TTY;

mod printer;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]  // turns off name mangling so we can easily link to it later.
pub extern "C" fn kernel_main() {
    let mut tty = TTY::new();
    tty.println("Testing testing");
    tty.println("Hello world");
}
