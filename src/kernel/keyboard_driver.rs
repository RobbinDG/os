use crate::{
    kernel::ports::{Port, read_port_byte},
    kernel::ps2::{KeyboardInitError, identity_devices},
};

const LOWER_CASE_OFFSET: u8 = 0x20;

/// A driver for a generic PS/2 connected keyboard.
pub struct KeyboardDriver {
    b1: u8,
    b2: u8,
    shift_offset: u8,
}

impl KeyboardDriver {
    /// Initialise the driver by reading the PS/2 connection and
    /// identifying the device for mapping inputs.
    pub unsafe fn initialise() -> Result<Self, KeyboardInitError> {
        unsafe {
            let (b1, b2) = identity_devices()?;
            Ok(Self {
                b1,
                b2,
                shift_offset: LOWER_CASE_OFFSET,
            })
        }
    }

    /// Handle a keyboard interrupt (IRQ1). This function will
    /// read the input on the data port and parse it.
    pub fn keyboard_interrupt_handler(&mut self) -> Option<u8> {
        let scan_code = read_port_byte(Port::PS2DataPort.into());
        self.letter_from_scan_code(scan_code)
    }

    fn letter_from_scan_code(&mut self, scan_code: u8) -> Option<u8> {
        match scan_code {
            0x01 => None, // Escape
            0x00..0x02 => Some(b'X'),
            0x02..=0x0b => Some(
                if self.shift_offset == LOWER_CASE_OFFSET {
                    b'0' + (scan_code - 0x02 + 1) % 10
                } else {
                    match scan_code {
                        0x02 => b'!',
                        0x03 => b'@',
                        0x04 => b'#',
                        0x05 => b'$',
                        0x06 => b'%',
                        0x07 => b'^',
                        0x08 => b'&',
                        0x09 => b'*',
                        0x0a => b'(',
                        0x0b => b')',
                        _ => b' '
                    }
                }                
            ),
            0x0c => Some(b'-'),
            0x0d => Some(b'='),
            0x0e => Some(0x08), // Backspace
            0x0f => Some('\t' as u8),
            0x10 => Some(b'Q' + self.shift_offset),
            0x11 => Some(b'W' + self.shift_offset),
            0x12 => Some(b'E' + self.shift_offset),
            0x13 => Some(b'R' + self.shift_offset),
            0x14 => Some(b'T' + self.shift_offset),
            0x15 => Some(b'Y' + self.shift_offset),
            0x16 => Some(b'U' + self.shift_offset),
            0x17 => Some(b'I' + self.shift_offset),
            0x18 => Some(b'O' + self.shift_offset),
            0x19 => Some(b'P' + self.shift_offset),
            0x1a => Some(b'['),
            0x1b => Some(b']'),
            0x1c => Some('\n' as u8),
            0x1e => Some(b'A' + self.shift_offset),
            0x1f => Some(b'S' + self.shift_offset),
            0x20 => Some(b'D' + self.shift_offset),
            0x21 => Some(b'F' + self.shift_offset),
            0x22 => Some(b'G' + self.shift_offset),
            0x23 => Some(b'H' + self.shift_offset),
            0x24 => Some(b'J' + self.shift_offset),
            0x25 => Some(b'K' + self.shift_offset),
            0x26 => Some(b'L' + self.shift_offset),
            0x27 => Some(b';'),
            0x28 => Some('\'' as u8),
            0x29 => Some(b'`'),
            0x2a => {
                self.shift_offset = 0;
                None
            } // Left shift
            0x2b => Some('\\' as u8),
            0x2c => Some(b'Z' + self.shift_offset),
            0x2d => Some(b'X' + self.shift_offset),
            0x2e => Some(b'C' + self.shift_offset),
            0x2f => Some(b'V' + self.shift_offset),
            0x30 => Some(b'B' + self.shift_offset),
            0x31 => Some(b'N' + self.shift_offset),
            0x32 => Some(b'M' + self.shift_offset),
            0x33 => Some(b','),
            0x34 => Some(b'.'),
            0x35 => Some(b'/'),
            0x36 => {
                self.shift_offset = 0;
                None // Right shift
            }
            0x39 => Some(b' '),
            0x3b..=0x44 => None, // F1 through F10
            0x53 => None,        // Delete
            0x57..=0x58 => None, // F11, F12
            0x1c..0x80 => Some(b'X'),
            // Release keys
            0xAA | 0xB6 => {
                self.shift_offset = LOWER_CASE_OFFSET;
                None
            }
            0x80..=0xFF => None, // Release
        }
    }
}
