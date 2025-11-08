use crate::{ports, vga};

const VIDEO_MEM: *mut u8 = 0xb8000 as *mut u8;

pub struct TTY {
    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

impl TTY {
    pub fn new() -> Self {
        TTY {
            width: 80,
            height: 25,
            x: 0,
            y: 0,
        }
    }

    pub unsafe fn clear(&mut self) {
        let screen_size = self.width * self.height;
        for i in 0..screen_size {
            unsafe {
                let addr = VIDEO_MEM.add(i as usize * 2);
                *addr = ' ' as u8;
            }
        }
        self.x = 0;
        self.y = 0;
        unsafe {
            self.update_cursor_position();
        }
    }

    pub unsafe fn get_cursor_position(&self) -> usize {
        unsafe {
            ports::write_port_byte(vga::Port::VGA3Out as u16, vga::VGA::CursorHiByte as u8);
            let mut offset = (ports::read_port_byte(vga::Port::VGA3In as u16) as usize) << 8;
            ports::write_port_byte(vga::Port::VGA3Out as u16, vga::VGA::CursorLoByte as u8);
            offset += ports::read_port_byte(vga::Port::VGA3In as u16) as usize;
            offset * 2
        } 
    }

    pub unsafe fn update_cursor_position(&mut self) {
        let offset = 2 * (self.y * self.width + self.x);
        let hi = offset >> 8;
        let lo = offset & 0x00ff;
        unsafe {
            ports::write_port_byte(vga::Port::VGA3Out as u16, vga::VGA::CursorHiByte as u8);
            ports::write_port_byte(vga::Port::VGA3In as u16, hi as u8);
            ports::write_port_byte(vga::Port::VGA3Out as u16, vga::VGA::CursorLoByte as u8);
            ports::write_port_byte(vga::Port::VGA3In as u16, lo as u8);
        }
    }

    pub unsafe fn println_ascii(&mut self, s: &[u8]) {
        for c in s {
            unsafe {
                let char_addr: *mut u8 = VIDEO_MEM.add(2 * (self.y * self.width + self.x) as usize);
                let col_addr = char_addr.add(1);
                char_addr.write_unaligned(*c);
                col_addr.write_unaligned(0x0f);
            }
            self.x = (self.x + 1) % self.width;
        }
        self.y = (self.y + 1) % self.height;
        self.x = 0;
        unsafe {
            self.update_cursor_position();
        }
    }

    pub unsafe fn print_hex(&mut self, n: u16) {
        let mut buf = [0; 4];
        buf[3] = Self::half_byte_to_hex_ascii(n & 0x000F);
        buf[2] = Self::half_byte_to_hex_ascii((n & 0x00F0) >> 4);
        buf[1] = Self::half_byte_to_hex_ascii((n & 0x0F00) >> 8);
        buf[0] = Self::half_byte_to_hex_ascii((n & 0xF000) >> 12);
        unsafe {
            self.println_ascii(&buf);
        }
    }

    /**
     * Convers the lowest half-byte to a hex ASCII character.
     */
    fn half_byte_to_hex_ascii(n: u16) -> u8 {
        if n <= 9 {
            '0' as u8 + n as u8
        } else {
            'A' as u8 + (n - 10) as u8
        }
    }
}
