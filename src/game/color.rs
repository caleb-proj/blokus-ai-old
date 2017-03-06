use std::mem;

#[derive(Copy, Clone)]
pub enum Color {
    Blue = 0,
    Yellow,
    Red,
    Green,
}

impl Color {
    pub fn next(&self) -> Color {
        unsafe { mem::transmute(((*self as u8) + 1) % 4) }
    }
}
