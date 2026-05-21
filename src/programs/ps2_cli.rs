use crate::console::Console;

pub unsafe fn ps2_cli(tty: &mut Console) {
    unsafe {
        tty.println_ascii("PS2 CLI reached.".as_bytes());
    }
}
