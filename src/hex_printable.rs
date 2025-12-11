use crate::mem::{DynArray, OutOfBounds};

pub trait HexPrintable {
    type Item;

    fn half_byte_to_hex_ascii(n: u8) -> u8 {
        if n <= 9 {
            '0' as u8 + n
        } else {
            'A' as u8 + (n - 10)
        }
    }

    unsafe fn convert_to_bytes(&self, buf: &mut DynArray<u8>) -> Result<(), OutOfBounds>;

    unsafe fn as_hex(&self) -> Result<DynArray<u8>, OutOfBounds> {
        unsafe {
            let byte_count = core::mem::size_of::<Self::Item>();
            let mut buf = DynArray::new(byte_count, false);
            self.convert_to_bytes(&mut buf)?;
            let mut hex_chars = DynArray::new(byte_count * 2, false);
            for i in 0..byte_count {
                let byte = buf.get(i)?;
                hex_chars.set(i * 2, Self::half_byte_to_hex_ascii(byte >> 4))?;
                hex_chars.set(i * 2 + 1, Self::half_byte_to_hex_ascii(byte & 0x0F))?;
            }
            Ok(hex_chars)
        }
    }
}

impl HexPrintable for u8 {
    type Item = u8;
    
    unsafe fn convert_to_bytes(&self, buf: &mut DynArray<u8>) -> Result<(), OutOfBounds> {
        unsafe {
            buf.set(0, *self)?;
            Ok(())
        }
    }
}

impl HexPrintable for u16 {
    type Item = u16;
    
    unsafe fn convert_to_bytes(&self, buf: &mut DynArray<u8>) -> Result<(), OutOfBounds> {
        unsafe {
            buf.set(1, (self & 0x00FF) as u8)?;
            buf.set(0, (self >> 8) as u8)?;
            Ok(())
        }
    }
}

impl HexPrintable for u32 {
    type Item = u32;

    unsafe fn convert_to_bytes(&self, buf: &mut DynArray<u8>) -> Result<(), OutOfBounds> {
        unsafe {
            buf.set(3, ((self >> 0) & 0xFF) as u8)?;
            buf.set(2, ((self >> 8) & 0xFF) as u8)?;
            buf.set(1, ((self >> 16) & 0xFF) as u8)?;
            buf.set(0, (self >> 24) as u8)?;
            Ok(())
        }
    }
}
