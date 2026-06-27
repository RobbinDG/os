use core::assert;

const BUF_SIZE: usize = 32;

use stdlib::print_ascii;
use stdlib::print_decimal;
use stdlib::print_hex;
use stdlib::println_ascii;
use stdlib::static_str::StaticString;

use crate::ps2_cli::ps2_cli;

enum Command {
    Empty,
    PS2,
    Mem,
    Commands,
}
pub struct Shell {
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

impl Shell {
    pub unsafe fn new() -> Self {
        let mut self_ = Self {
            buf: StaticString::new(0),
            cmds: [
                (make_command(""), Command::Empty),
                (make_command("ps2"), Command::PS2),
                (make_command("commands"), Command::Commands),
                (make_command("mem"), Command::Mem),
            ],
        };
        unsafe {
            print_ascii("\x1b[1J".as_bytes());
            self_.print_flair()
        };
        self_
    }

    pub unsafe fn handle_key(&mut self, key: u8) {
        unsafe {
            print_ascii(&[key]);
            match key {
                0x08 => {
                    if self.buf.len() > 0 {
                        self.buf.pop();
                    }
                }
                0x0A => {
                    self.execute_command();
                }
                _ => {
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
                            Command::Empty => {}
                            Command::PS2 => ps2_cli(),
                            Command::Commands => self.print_cmd_options(),
                            Command::Mem => {} //self.print_mem(),
                        }
                    }
                    return;
                }
            }
        }
        println_ascii("Command not recognised.".as_bytes());
    }

    unsafe fn execute_command(&mut self) {
        unsafe {
            self.execute_command_in_buffer();
            self.buf.clear();
            self.print_flair();
        }
    }

    unsafe fn print_flair(&mut self) {
        print_ascii("$> ".as_bytes());
    }

    unsafe fn print_cmd_options(&mut self) {
        for (cmd_str, _) in &self.cmds {
            println_ascii(cmd_str);
        }
    }

    /*
    unsafe fn print_mem(&mut self) {
        unsafe {
            KERNEL.mem.with_unwrap(|mem_mgr| {
                let mem = mem_mgr.get_memory();
                print_ascii("Low mem size: ".as_bytes());
                print_decimal(mem.low_mem_size);
                println_ascii(" kb".as_bytes());
                for hm in mem.high_mem {
                    match hm {
                        Some(entry) => {
                            print_hex(entry.base);
                            print_ascii(" - ".as_bytes());
                            print_hex(entry.len);
                            print_ascii(" - ".as_bytes());
                            print_hex::<u8>((&entry.typ).into());
                            println_ascii(&[]);
                        }
                        None => return,
                    }
                }
            });
        }
    }
    */
}
