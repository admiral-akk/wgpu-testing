use crate::structs::{color::Color, dimensions::Dimensions};

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
