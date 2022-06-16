use color::Color;
use dimensions::Dimensions;
use gpu::{basic_compute, draw_uv::draw_uv, gpu::GPU};
use utils::write_image::write_image;

use crate::gpu::copy_val;
mod color;
pub mod dimensions;
mod gpu;
mod utils;

pub fn write_test_image(dimensions: &Dimensions) {
    let colors = Color::test_uv(&dimensions);
    write_image(dimensions, &colors, "test_image");
}

pub fn copy_via_gpu(input: Vec<u32>) -> Vec<u32> {
    let gpu: GPU = pollster::block_on(GPU::new());
    return copy_val::copy_val(&gpu, &input);
}

pub fn apply_basic_compute_shader(input: Vec<u32>) -> Vec<u32> {
    let gpu: GPU = pollster::block_on(GPU::new());
    return basic_compute::basic_compute(&gpu, &input);
}

pub fn write_test_image_via_gpu(dimensions: &Dimensions) {
    let gpu: GPU = pollster::block_on(GPU::new());
    let colors = draw_uv(&gpu, dimensions);
    write_image(dimensions, &colors, "gpu_uv");
}

#[test]
fn cpu_gpu_uv_match() {
    let dimensions = Dimensions::new(240, 300);
    let gpu: GPU = pollster::block_on(GPU::new());

    let gpu_color = draw_uv(&gpu, &dimensions);
    let cpu_color = Color::test_uv(&dimensions);
    for i in 0..dimensions.size() {
        assert_eq!(cpu_color[i], gpu_color[i]);
    }
}
