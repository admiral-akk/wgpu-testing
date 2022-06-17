use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Color {
    rgba: u32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgba: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 255,
        }
    }
    pub fn new32(rgba: u32) -> Self {
        Self { rgba }
    }

    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (
            ((self.rgba >> 24) & 255) as u8,
            ((self.rgba >> 16) & 255) as u8,
            ((self.rgba >> 8) & 255) as u8,
            (self.rgba & 255) as u8,
        )
    }

    pub fn to_rgb(&self) -> String {
        format!(
            "{} {} {}",
            (self.rgba >> 24) & 255,
            (self.rgba >> 16) & 255,
            (self.rgba >> 8) & 255
        )
    }
}
