use core::ops::{Div, Rem};

use crate::{KERNEL, dyn_array::DynArray, kernel::kernel::KernelError};

pub trait DecimalDigits {
    fn decimal_digits() -> usize;

    fn extract_low_byte(self) -> u8;
}

pub trait DecimalPrintable {
    unsafe fn as_decimal<'a>(&'a self) -> Result<DynArray<'a, u8>, KernelError>;
}

impl<T> DecimalPrintable for T
where
    T: Div<T, Output = T> + Rem<T, Output = T> + Eq + Copy + From<u8> + DecimalDigits,
{
    unsafe fn as_decimal<'a>(&'a self) -> Result<DynArray<'a, u8>, KernelError> {
        unsafe {
            let byte_count = Self::decimal_digits();
            let mem = KERNEL.get()?.memory_manager();
            let mut chars = DynArray::new(byte_count, false, mem);
            let mut remainder = *self;
            let ten = T::from(10);
            let zero = T::from(0);
            for i in 0..byte_count {
                if remainder == zero {
                    break;
                }
                let digit = (remainder % ten).extract_low_byte();
                remainder = remainder / ten;
                chars.set(i, digit + b'0')?;
            }
            Ok(chars)
        }
    }
}

impl DecimalDigits for u8 {
    fn decimal_digits() -> usize {
        3
    }

    fn extract_low_byte(self) -> u8 {
        self
    }
}

impl DecimalDigits for u16 {
    fn decimal_digits() -> usize {
        5
    }

    fn extract_low_byte(self) -> u8 {
        self as u8
    }
}

impl DecimalDigits for u32 {
    fn decimal_digits() -> usize {
        10
    }

    fn extract_low_byte(self) -> u8 {
        self as u8
    }
}

impl DecimalDigits for u64 {
    fn decimal_digits() -> usize {
        20
    }

    fn extract_low_byte(self) -> u8 {
        self as u8
    }
}

/*
impl DecimalPrintable for u8 {
    type Item = u8;

    unsafe fn convert_to_bytes(&self, buf: &mut DynArray<u8>) -> Result<(), KernelError> {
        unsafe {
            buf.set(0, *self)?;
            Ok(())
        }
    }
}

impl DecimalPrintable for u16 {
    type Item = u16;

    unsafe fn convert_to_bytes(&self, buf: &mut DynArray<u8>) -> Result<(), KernelError> {
        unsafe {
            buf.set(1, (self & 0x00FF) as u8)?;
            buf.set(0, (self >> 8) as u8)?;
            Ok(())
        }
    }
}

impl DecimalPrintable for u32 {
    type Item = u32;

    unsafe fn convert_to_bytes(&self, buf: &mut DynArray<u8>) -> Result<(), KernelError> {
        unsafe {
            buf.set(3, ((self >> 0) & 0xFF) as u8)?;
            buf.set(2, ((self >> 8) & 0xFF) as u8)?;
            buf.set(1, ((self >> 16) & 0xFF) as u8)?;
            buf.set(0, (self >> 24) as u8)?;
            Ok(())
        }
    }
}
*/
