#[derive(Default, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_rgb(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}
