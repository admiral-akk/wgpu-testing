use std::{fs::File, io::Write};

use crate::{color::Color, dimensions::Dimensions};

pub struct ImageWriter {
    dimensions: Dimensions,
    colors: Vec<Color>,
}

impl ImageWriter {
    pub fn new(dimensions: Dimensions) -> Self {
        let colors: Vec<Color> = vec![Color::default(); dimensions.size()];
        Self { dimensions, colors }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        *self.get_color(x, y) = color;
    }

    fn get_color(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.colors[self.dimensions.index(x, y)]
    }

    pub fn write_image(&mut self) -> std::io::Result<()> {
        let mut file = File::create("image.ppm")?;
        file.write(
            format!(
                "P3\n{} {}\n255\n",
                self.dimensions.width, self.dimensions.height
            )
            .as_bytes(),
        )
        .expect("Failed to write header!");
        for y in (0..self.dimensions.height).rev() {
            for x in 0..self.dimensions.width {
                file.write(format!("{}\n", self.get_color(x, y).to_rgb()).as_bytes())
                    .expect("Failed to write color!");
            }
        }
        Ok(())
    }
}
