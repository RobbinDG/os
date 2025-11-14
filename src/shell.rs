use crate::{printer::VGAText, static_str::StaticString};

const BUF_SIZE: usize = 32;

pub struct Shell {
    tty: VGAText,
    buf: StaticString<BUF_SIZE, u8>
}

impl Shell {
    pub fn new(tty: VGAText) -> Self {
        Self {
            tty,
            buf: StaticString::new(0),
        }
    }

    pub unsafe fn handle_key(&mut self, key: u8) {
        unsafe {
            match key {
                0x08 => {
                    if self.buf.len() > 0 {
                        self.tty.bs();
                    }
                }
                0x0A => {
                    self.tty.nl();
                    self.execute_command();
                }
                _ => {
                    self.tty.put_char(key);
                    self.buf.push(key);
                }
            }
        }
    }

    unsafe fn execute_command(&mut self) {
        unsafe {
            self.tty.print_ascii("Executing: ".as_bytes());
            self.tty.print_ascii(&self.buf.make_printable());
            self.buf.clear();
        }
    }
}
