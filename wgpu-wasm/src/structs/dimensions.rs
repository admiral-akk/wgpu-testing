use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: (width as u32),
            height: (height as u32),
        }
    }

    pub fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn index(&self, x: u32, y: u32) -> usize {
        (self.width * y + x) as usize
    }
}
