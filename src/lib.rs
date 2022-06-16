use gpu::{basic_compute, draw_uv::draw_uv, gpu::GPU};
use structs::dimensions::Dimensions;
use utils::write_image::write_image;

use crate::gpu::copy_val;
use utils::test_uv::test_uv;
mod gpu;
pub mod structs;
mod utils;

pub fn write_test_image(dimensions: &Dimensions) {
    let colors = test_uv(dimensions);
    write_image(dimensions, &colors, "test_image").expect("CPU Test Image Failed");
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
    write_image(dimensions, &colors, "gpu_uv").expect("GPU Test Image Failed");
}

#[test]
fn cpu_gpu_uv_match() {
    let dimensions = Dimensions::new(240, 300);
    let gpu: GPU = pollster::block_on(GPU::new());

    let gpu_color = draw_uv(&gpu, &dimensions);
    let cpu_color = test_uv(&dimensions);
    for i in 0..dimensions.size() {
        assert_eq!(cpu_color[i], gpu_color[i]);
    }
}
