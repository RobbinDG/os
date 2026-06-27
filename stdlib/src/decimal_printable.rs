use core::ops::{Div, Rem};

use crate::{KernelError, Vec};

pub trait DecimalDigits {
    fn decimal_digits() -> usize;

    fn extract_low_byte(self) -> u8;
}

pub trait DecimalPrintable {
    fn as_decimal<'a>(&'a self) -> Result<Vec<u8>, KernelError>;
}

impl<T> DecimalPrintable for T
where
    T: Div<T, Output = T> + Rem<T, Output = T> + Eq + Copy + From<u8> + DecimalDigits,
{
    fn as_decimal<'a>(&'a self) -> Result<Vec<u8>, KernelError> {
        let byte_count = Self::decimal_digits();
        let mut chars = Vec::<u8>::malloc(byte_count)?;
        let mut remainder = *self;
        let ten = T::from(10);
        let zero = T::from(0);
        for i in 0..byte_count {
            if remainder == zero {
                break;
            }
            let digit = (remainder % ten).extract_low_byte();
            remainder = remainder / ten;
            chars[i] = digit + b'0';
        }
        Ok(chars)
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
