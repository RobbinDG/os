use crate::{printer::VGAText, static_str::StaticString};

const BUF_SIZE: usize = 32;

pub struct Shell {
    tty: VGAText,
    buf: StaticString<BUF_SIZE, u8>,
}

const CMD_PS2: [u8; 3] = [b'P', b'S', b'2'];

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

    unsafe fn execute_command_in_buffer(&mut self) {
        let cmd = self.buf.make_printable();

        for i in 0..cmd.len() {
            if i >= CMD_PS2.len() {
                unsafe {
                    self.run_cmd_ps2();
                }
                return;
            }
            if cmd[i] != CMD_PS2[i] {
                return;
            }
        }
    }

    unsafe fn run_cmd_ps2(&mut self) {
        unsafe {
            self.tty.print_ascii("PS2 cmd!".as_bytes());
            self.tty.nl();
        }
    }

    unsafe fn execute_command(&mut self) {
        unsafe {
            self.tty.print_ascii("Executing: ".as_bytes());
            self.tty.print_ascii(&self.buf.make_printable());
            self.execute_command_in_buffer();
            self.buf.clear();
        }
    }
}
