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
            height: 20,
            x: 0,
            y: 0,
        }
    }

    pub fn println(&mut self, s: &str) {
        for c in s.as_bytes() {
            unsafe {
                let char_addr: *mut u8 = VIDEO_MEM.add(2 * (self.y * self.width + self.x) as usize);
                let col_addr = char_addr.add(1);
                *char_addr = *c;
                *col_addr = 0x0f;
            }
            self.x = (self.x + 1) % self.width;
        }
        self.y = (self.y + 1) % self.height;
        self.x = 0;
    }
}
