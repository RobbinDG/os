use crate::{
    KERNEL,
    vga_driver::{HEIGHT, WIDTH},
};

static mut X: u16 = 0;
static mut Y: u16 = 0;
static mut ACTIVE: bool = false;

/// A virtual console implementation that can be controlled using ANSI control codes.
///
/// Under the hood, a global state is maintained that
/// contains the cursor position. When an instance is created,
/// it copies this state, storing it back when dropped. This circumvents
/// problems with mutably borrowing static mutables and follows borrowing
/// rules.
pub struct Console {
    x: u16,
    y: u16,
}

impl Console {
    /// Creates a new instance using a driver. The state is not synchronized and
    /// will overwrite existing text when needed.
    pub unsafe fn create() -> Self {
        unsafe { Self { x: X, y: Y } }
    }

    /// Obtains an instance of the TTY, if one has not been used yet.
    /// The returned Option acts as a non-blocking lock, returning `None`
    /// when an instance is already in use.
    pub unsafe fn get_instance() -> Option<Self> {
        unsafe { if ACTIVE { None } else { Some(Self::create()) } }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe {
            X = self.x;
            Y = self.y;
            ACTIVE = false;
        }
    }
}

impl Console {
    pub unsafe fn write_ansi(&mut self, chars: &[u8]) {
        let mut esc_param = [0u8; 4];
        let mut esc_intermediate = [0u8; 4];
        let mut esc_param_idx = 0usize;
        let mut esc_intermediate_idx = 0usize;
        let mut in_esc = false;
        for i in 0..chars.len() {
            // CSI commands
            if in_esc {
                match chars[i] {
                    b'[' => {
                        esc_param_idx = 0;
                        esc_intermediate_idx = 0;
                    }
                    0x30..=0x3F => {
                        esc_param[esc_param_idx] = chars[i];
                        esc_param_idx += 1;
                    }
                    0x20..=0x2F => {
                        esc_intermediate[esc_intermediate_idx] = chars[i];
                        esc_intermediate_idx += 1;
                    }
                    0x40..=0x7E => {
                        match chars[i] {
                            b'J' => {
                                unsafe { self.clear() };
                            }
                            _ => {} // Unknown code, skip.
                        }
                        in_esc = false;
                    }
                    _ => {} // Skip any non-escape char
                }
                continue;
            }

            // Non-control characters
            if !char::is_ascii_control(&(chars[i] as char)) {
                unsafe { self.put_char(chars[i]) };
                continue;
            }

            // C0 control codes
            unsafe {
                match chars[i] {
                    0x08 => {
                        self.bs();
                        continue;
                    }
                    0x09 => {
                        // Tab
                        continue;
                    }
                    0x0A => {
                        // Line feed
                        self.nl();
                        continue;
                    }
                    0x0D => {
                        // Carriage return
                        self.move_cursor(0, 0);
                        continue;
                    }
                    0x1B => {
                        in_esc = true;
                        continue;
                    }
                    _ => {} // I hate this.
                }
            }
        }
    }

    pub unsafe fn clear(&mut self) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                for i in 0..HEIGHT {
                    vga.clear_row(i);
                }
                self.x = 0;
                self.y = 0;
                vga.update_cursor_position(self.x, self.y);
            })
        };
    }

    pub unsafe fn put_char(&mut self, c: u8) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                vga.put_char_raw(c, self.x, self.y);
                self.move_cursor(1, 0);
                vga.update_cursor_position(self.x, self.y);
            });
        }
    }

    pub unsafe fn scroll(&mut self, columns: u16) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                for i in 0..HEIGHT {
                    if i + columns < HEIGHT {
                        vga.copy_row(i + columns, i);
                    } else {
                        vga.clear_row(i);
                    }
                }
            });
        }
    }

    pub unsafe fn bs(&mut self) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                vga.put_char_raw(b' ', self.x - 1, self.y);
                self.move_cursor(-1, 0);
                vga.update_cursor_position(self.x, self.y);
            });
        }
    }

    pub fn nl(&mut self) {
        unsafe {
            self.move_cursor(0, 1);
        }
        self.x = 0;
        unsafe {
            KERNEL
                .vga_driver
                .with_unwrap(|vga| vga.update_cursor_position(self.x, self.y))
        };
    }

    unsafe fn move_cursor(&mut self, dx: i16, dy: i16) {
        let x_acc = self.x.wrapping_add_signed(dx);
        self.x = x_acc % WIDTH;
        let mut new_y = self.y.wrapping_add_signed(dy) + x_acc / WIDTH;
        if new_y >= HEIGHT {
            let diff = new_y.wrapping_sub(HEIGHT - 1);
            unsafe { self.scroll(diff) };
            new_y = new_y.wrapping_sub(diff);
        }
        self.y = new_y;
    }
}
