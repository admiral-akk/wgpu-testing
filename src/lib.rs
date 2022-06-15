use color::Color;
use dimensions::Dimensions;
use image_writer::ImageWriter;

mod color;
mod dimensions;
mod image_writer;

pub fn write_test_image() {
    let dimensions = Dimensions::new(200, 200);
    let mut image = ImageWriter::new(dimensions);
    for x in 0..dimensions.width {
        for y in 0..dimensions.height {
            let color = Color::new(
                (255 * x / dimensions.width) as u8,
                (255 * y / dimensions.height) as u8,
                0,
            );
            image.set_color(x, y, color);
        }
    }
    image.write_image().expect("Writing image failed!");
}
