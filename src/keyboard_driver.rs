use crate::{
    ports::{Port, read_port_byte},
    ps2::{KeyboardInitError, identity_devices},
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
            0x00..0x02 => Some('X' as u8),
            0x02 => Some('1' as u8),
            0x03 => Some('2' as u8),
            0x04 => Some('3' as u8),
            0x05 => Some('4' as u8),
            0x06 => Some('5' as u8),
            0x07 => Some('6' as u8),
            0x08 => Some('7' as u8),
            0x09 => Some('8' as u8),
            0x0a => Some('9' as u8),
            0x0b => Some('0' as u8),
            0x0c => Some('-' as u8),
            0x0d => Some('=' as u8),
            0x0e => Some(0x08), // Backspace
            0x0f => Some('\t' as u8),
            0x10 => Some('Q' as u8 + self.shift_offset),
            0x11 => Some('W' as u8 + self.shift_offset),
            0x12 => Some('E' as u8 + self.shift_offset),
            0x13 => Some('R' as u8 + self.shift_offset),
            0x14 => Some('T' as u8 + self.shift_offset),
            0x15 => Some('Y' as u8 + self.shift_offset),
            0x16 => Some('U' as u8 + self.shift_offset),
            0x17 => Some('I' as u8 + self.shift_offset),
            0x18 => Some('O' as u8 + self.shift_offset),
            0x19 => Some('P' as u8 + self.shift_offset),
            0x1a => Some('[' as u8),
            0x1b => Some(']' as u8),
            0x1c => Some('\n' as u8),
            0x1e => Some('A' as u8 + self.shift_offset),
            0x1f => Some('S' as u8 + self.shift_offset),
            0x20 => Some('D' as u8 + self.shift_offset),
            0x21 => Some('F' as u8 + self.shift_offset),
            0x22 => Some('G' as u8 + self.shift_offset),
            0x23 => Some('H' as u8 + self.shift_offset),
            0x24 => Some('J' as u8 + self.shift_offset),
            0x25 => Some('K' as u8 + self.shift_offset),
            0x26 => Some('L' as u8 + self.shift_offset),
            0x27 => Some(';' as u8),
            0x28 => Some('\'' as u8),
            0x29 => Some('`' as u8),
            0x2a => {
                self.shift_offset = 0;
                None
            } // Left shift
            0x2b => Some('\\' as u8),
            0x2c => Some('Z' as u8 + self.shift_offset),
            0x2d => Some('X' as u8 + self.shift_offset),
            0x2e => Some('C' as u8 + self.shift_offset),
            0x2f => Some('V' as u8 + self.shift_offset),
            0x30 => Some('B' as u8 + self.shift_offset),
            0x31 => Some('N' as u8 + self.shift_offset),
            0x32 => Some('M' as u8 + self.shift_offset),
            0x33 => Some(',' as u8),
            0x34 => Some('.' as u8),
            0x35 => Some('/' as u8),
            0x36 => {
                self.shift_offset = 0;
                None // Right shift
            }
            0x39 => Some(' ' as u8),
            0x3b..=0x44 => None, // F1 through F10
            0x53 => None,        // Delete
            0x57..=0x58 => None, // F11, F12
            0x1c..0x80 => Some('X' as u8),
            // Release keys
            0xAA | 0xB6 => {
                self.shift_offset = LOWER_CASE_OFFSET;
                None
            }
            0x80..=0xFF => None, // Release
        }
    }
}
