use core::cmp::min;

use crate::KERNEL;

// Choose power of 2 for modulus efficiency.
const BUF_SIZE: usize = 64;

pub struct TTY<D>
where
    D: Copy + Default,
{
    input_buf: [D; BUF_SIZE],
    input_idx: usize,
    input_size: usize,
    // output_buf: [D; BUF_SIZE],
    // output_idx: usize,
    // output_size: usize,
    echo: bool,
}

impl<D> TTY<D>
where
    D: Copy + Default,
{
    pub fn new(echo: bool) -> Self {
        Self {
            input_buf: [D::default(); BUF_SIZE],
            input_idx: 0,
            input_size: 0,
            echo,
        }
    }

    /// Read a fixed amount of input characters into a buffer.
    pub fn read(&mut self, dest: &mut [D], amount: usize) -> usize {
        let actual_amount = min(amount, self.input_size);
        for i in 0..actual_amount {
            dest[i] = self.input_buf[self.input_idx];
            self.input_idx = (self.input_idx + 1) % BUF_SIZE;
        }
        self.input_size -= actual_amount;
        actual_amount
    }

    pub fn receive_input(&mut self, data: D) {
        self.input_buf[self.input_idx] = data;
        self.input_idx = (self.input_idx + 1) % BUF_SIZE;
        self.input_size = min(self.input_size + 1, BUF_SIZE);
        // if self.echo {
        //     self.write(data);
        // }
    }
}

/// Temporary implementation of a simple write function such that we can get
/// the TTY pipeline to work initially. Definitely improve and generalise later.
impl TTY<u8> {
    pub unsafe fn write(&mut self, data: u8) -> usize {
        unsafe { KERNEL.vga_driver.with_unwrap(|vga| vga.put_char_raw(data, 0, 0)) };
        0
    }
}
