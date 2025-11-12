use crate::{
    ports::{Port, read_port_byte},
    printer::TTY,
    ps2::identity_devices,
};

pub struct KeyboardDriver {
    b1: u8,
    b2: u8,
}

impl KeyboardDriver {
    pub unsafe fn initialise() -> Result<Self, ()> {
        unsafe {
            let (b1, b2) = identity_devices()?;
            Ok(Self { b1, b2 })
        }
    }

    pub unsafe fn keyboard_interrupt_handler(&mut self) {
        unsafe {
            let scan_code = read_port_byte(Port::PS2DataPort.into());
            if let Some(mut tty) = TTY::get_instance() {
                tty.print_ascii(Self::letter_from_scan_code(scan_code).as_bytes());
                tty.print_hex(scan_code as u16);
            }
        }
    }

    fn letter_from_scan_code(scan_code: u8) -> &'static str {
        match scan_code {
            0x10 => "q",
            _ => "X",
        }
    }
}
