use crate::printer::VGAText;

pub unsafe fn ps2_cli(tty: &mut VGAText) {
    unsafe {
        tty.println_ascii("PS2 CLI reached.".as_bytes());
    }
}
