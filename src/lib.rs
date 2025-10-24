#![no_std]  // don’t use the Rust standard library
#![no_main] // you’ll provide your own entry point (optional)

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]  // turns off name mangling so we can easily link to it later.
pub extern "C" fn kernel_main() -> u16 {
    // your kernel entry point
    // loop {}
    0xbeef    
}
