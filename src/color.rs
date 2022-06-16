use bytemuck::{Pod, Zeroable};

use crate::dimensions::Dimensions;

#[derive(Clone, Copy, Pod, Zeroable, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Color {
    rgba: u32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgba: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8),
        }
    }

    pub fn to_rgb(&self) -> String {
        format!(
            "{} {} {}",
            (self.rgba >> 24) & 255,
            (self.rgba >> 16) & 255,
            (self.rgba >> 8) & 255
        )
    }

    pub fn test_uv(dimensions: &Dimensions) -> Vec<Color> {
        let mut colors: Vec<Color> = Vec::new();
        for y in 0..dimensions.height {
            for x in 0..dimensions.width {
                let color = Color::new(
                    (255 * x / dimensions.width) as u8,
                    (255 * y / dimensions.height) as u8,
                    0,
                );
                colors.push(color);
            }
        }
        return colors;
    }
}
