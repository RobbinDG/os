use crate::{kernel::kernel::KernelError, programs::lib::Vec};

pub trait HexPrintable {
    type Item;

    fn half_byte_to_hex_ascii(n: u8) -> u8 {
        if n <= 9 { b'0' + n } else { b'A' + (n - 10) }
    }

    unsafe fn convert_to_bytes(&self, buf: &mut Vec<u8>) -> Result<(), KernelError>;

    unsafe fn as_hex<'a>(&'a self) -> Result<Vec<u8>, KernelError> {
        unsafe {
            let byte_count = core::mem::size_of::<Self::Item>();
            let mut buf = Vec::<u8>::malloc(byte_count)?;
            self.convert_to_bytes(&mut buf)?;
            let mut hex_chars = Vec::<u8>::malloc(byte_count * 2)?;
            for i in 0..byte_count {
                let byte = buf[i];
                hex_chars[i * 2] = Self::half_byte_to_hex_ascii(byte >> 4);
                hex_chars[i * 2 + 1] = Self::half_byte_to_hex_ascii(byte & 0x0F);
            }
            Ok(hex_chars)
        }
    }
}

impl HexPrintable for u8 {
    type Item = u8;

    unsafe fn convert_to_bytes(&self, buf: &mut Vec<u8>) -> Result<(), KernelError> {
        buf[0] = *self;
        Ok(())
    }
}

impl HexPrintable for u16 {
    type Item = u16;

    unsafe fn convert_to_bytes(&self, buf: &mut Vec<u8>) -> Result<(), KernelError> {
        buf[1] = (self & 0x00FF) as u8;
        buf[0] = (self >> 8) as u8;
        Ok(())
    }
}

impl HexPrintable for u32 {
    type Item = u32;

    unsafe fn convert_to_bytes(&self, buf: &mut Vec<u8>) -> Result<(), KernelError> {
        buf[3] = ((self >> 0) & 0xFF) as u8;
        buf[2] = ((self >> 8) & 0xFF) as u8;
        buf[1] = ((self >> 16) & 0xFF) as u8;
        buf[0] = (self >> 24) as u8;
        Ok(())
    }
}

impl HexPrintable for u64 {
    type Item = u64;

    unsafe fn convert_to_bytes(&self, buf: &mut Vec<u8>) -> Result<(), KernelError> {
        buf[7] = ((self >> 0) & 0xFF) as u8;
        buf[6] = ((self >> 8) & 0xFF) as u8;
        buf[5] = ((self >> 16) & 0xFF) as u8;
        buf[4] = ((self >> 24) & 0xFF) as u8;
        buf[3] = ((self >> 32) & 0xFF) as u8;
        buf[2] = ((self >> 40) & 0xFF) as u8;
        buf[1] = ((self >> 48) & 0xFF) as u8;
        buf[0] = (self >> 56) as u8;
        Ok(())
    }
}
