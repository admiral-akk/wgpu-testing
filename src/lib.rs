use color::Color;
use dimensions::Dimensions;
use gpu::gpu::GPU;
use image_writer::ImageWriter;

use crate::gpu::copy_val;
mod color;
mod dimensions;
mod gpu;
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

pub fn copy_via_gpu(input: Vec<u8>) -> Vec<u8> {
    let gpu: GPU = pollster::block_on(GPU::new());
    return copy_val::copy_val(&gpu, &input);
}

pub fn apply_basic_compute_shader(input: Vec<u8>) -> Vec<u8> {
    return input;
}
