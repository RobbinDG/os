
pub enum Port {
    VGA1Out = 0x3C4,
    VGA1In = 0x3C5,
    VGA2Out = 0x3CE,
    VGA2In = 0x3CF,
    VGA3Out = 0x3D4,
    VGA3In = 0x3D5,
}

pub enum VGA {
    CursorHiByte = 0x0E,
    CursorLoByte = 0x0F,
}