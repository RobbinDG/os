use crate::printer::VGATextWriter;

pub unsafe fn ps2_cli(tty: &mut VGATextWriter) {
    unsafe {
        tty.println_ascii("PS2 CLI reached.".as_bytes());
    }
}
