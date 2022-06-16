mod color;
mod dimensions;
mod image_writer;

use wgpu_testing::{apply_basic_compute_shader, copy_via_gpu, write_test_image};

fn main() {
    let input: Vec<u32> = vec![2, 4, 8, 16, 0, 4, 2];
    let output = apply_basic_compute_shader(input);
    for val in output {
        println!("{}", val);
    }
}
