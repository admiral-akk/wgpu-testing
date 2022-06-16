use std::{fs::File, io::Write};

use crate::{color::Color, dimensions::Dimensions};

pub fn write_image(
    dimensions: &Dimensions,
    colors: &Vec<Color>,
    file_name: &str,
) -> std::io::Result<()> {
    let mut file = File::create(format!("output/{}.ppm", file_name))?;
    file.write(format!("P3\n{} {}\n255\n", dimensions.width, dimensions.height).as_bytes())
        .expect("Failed to write header!");
    for y in (0..dimensions.height).rev() {
        for x in 0..dimensions.width {
            file.write(format!("{}\n", colors[dimensions.index(x, y)].to_rgb()).as_bytes())
                .expect("Failed to write color!");
        }
    }
    Ok(())
}
