use crate::{KERNEL, printer::VGATextWriter, programs::ps2_cli::ps2_cli, static_str::StaticString};

const BUF_SIZE: usize = 32;

enum Command {
    Empty,
    PS2,
    Mem,
    Commands,
}
pub struct Shell<'a> {
    tty: VGATextWriter<'a>,
    buf: StaticString<BUF_SIZE, u8>,
    cmds: [([u8; BUF_SIZE], Command); 4], // TODO this implementation needs work!
}

const fn make_command(command_str: &str) -> [u8; BUF_SIZE] {
    let bytes: &[u8] = command_str.as_bytes();
    assert!(bytes.len() <= BUF_SIZE);

    let mut out: [u8; BUF_SIZE] = [0u8; BUF_SIZE];
    let mut i = 0;
    while i < bytes.len() {    
        out[i] = bytes[i];
        i += 1;
    }
    out
}

impl<'a> Shell<'a> {
    pub unsafe fn new(tty: VGATextWriter<'a>) -> Self {
        let mut self_ = Self {
            tty,
            buf: StaticString::new(0),
            cmds: [
                (make_command(""), Command::Empty),
                (make_command("ps2"), Command::PS2),
                (make_command("commands"), Command::Commands),
                (make_command("mem"), Command::Mem),
            ],
        };
        unsafe { self_.print_flair() };
        self_
    }

    pub unsafe fn handle_key(&mut self, key: u8) {
        unsafe {
            match key {
                0x08 => {
                    if self.buf.len() > 0 {
                        self.tty.bs();
                        self.buf.pop();
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

        for (cmd_str, cmd_func) in &self.cmds {
            for i in 0..cmd.len() {
                if cmd[i] != cmd_str[i] {
                    break;
                }
                if cmd_str[i] == b'\0' {
                    unsafe {
                        match cmd_func {
                            Command::Empty => {},
                            Command::PS2 => ps2_cli(&mut self.tty),
                            Command::Commands => self.print_cmd_options(),
                            Command::Mem => self.print_mem(),
                        }
                    }
                    return;
                }
            }
        }
        unsafe {
            self.tty.println_ascii("Command not recognised.".as_bytes());
        }
    }

    unsafe fn execute_command(&mut self) {
        unsafe {
            self.execute_command_in_buffer();
            self.buf.clear();
            self.print_flair();
        }
    }

    unsafe fn print_flair(&mut self) {
        unsafe { self.tty.print_ascii("$> ".as_bytes()) };
    }

    unsafe fn print_cmd_options(&mut self) {
        unsafe {
            for (cmd_str, _) in &self.cmds {
                self.tty.println_ascii(cmd_str);
            }
        }
    }

    unsafe fn print_mem(&mut self) {
        unsafe {
            match KERNEL.get() {
                Ok(k) => {
                    let mem = k.memory_manager().lock().get_memory();

                    self.tty.print_ascii("Low mem size: ".as_bytes());
                    self.tty.print_decimal(mem.low_mem_size);
                    self.tty.println_ascii(" kb".as_bytes());
                    self.tty.nl();
                    for hm in mem.high_mem {
                        match hm {
                            Some(entry) => {
                                self.tty.print_hex(entry.base);
                                self.tty.print_ascii(" - ".as_bytes());
                                self.tty.print_hex(entry.len);
                                self.tty.print_ascii(" - ".as_bytes());
                                self.tty.print_hex::<u8>((&entry.typ).into());
                                self.tty.nl();
                            }
                            None => return,
                        }
                    }
                }
                Err(_) => self.tty.println_ascii("Kernel Error.".as_bytes()),
            }
        }
    }
}
