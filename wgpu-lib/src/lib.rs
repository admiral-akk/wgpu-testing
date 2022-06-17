use examples::{basic_compute, copy_val::copy_val, draw_uv::draw_uv};
use gpu::gpu::GPU;
use structs::{color::Color, dimensions::Dimensions};
use utils::write_image::write_image;

use utils::test_uv::test_uv;
mod examples;
mod gpu;
pub mod structs;
mod utils;

pub fn write_test_image(dimensions: &Dimensions) {
    let colors = test_uv(dimensions);
    write_image(dimensions, &colors, "test_image").expect("CPU Test Image Failed");
}

pub fn get_colors(dimensions: &Dimensions) -> Vec<Color> {
    test_uv(dimensions)
}

pub async fn get_colors_gpu(dimensions: &Dimensions) -> Vec<Color> {
    let gpu: GPU = GPU::new().await;
    draw_uv(&gpu, &dimensions).await
}

pub async fn copy_via_gpu(input: Vec<u32>) -> Vec<u32> {
    let gpu: GPU = GPU::new().await;
    return copy_val(&gpu, &input).await;
}

pub async fn apply_basic_compute_shader(input: Vec<u32>) -> Vec<u32> {
    let gpu: GPU = GPU::new().await;
    return basic_compute::basic_compute(&gpu, &input).await;
}

pub async fn write_test_image_via_gpu(dimensions: &Dimensions) {
    let gpu: GPU = GPU::new().await;
    let colors = draw_uv(&gpu, dimensions).await;
    write_image(dimensions, &colors, "gpu_uv").expect("GPU Test Image Failed");
}

#[test]
fn cpu_gpu_uv_match() {
    let dimensions = Dimensions::new(240, 300);
    let gpu: GPU = pollster::block_on(GPU::new());

    let gpu_color = pollster::block_on(draw_uv(&gpu, &dimensions));
    let cpu_color = test_uv(&dimensions);
    for i in 0..dimensions.size() {
        assert_eq!(cpu_color[i], gpu_color[i]);
    }
}
