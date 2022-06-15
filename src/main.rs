mod color;
mod dimensions;
mod image_writer;

use wgpu_testing::{apply_basic_compute_shader, copy_via_gpu, write_test_image};

fn main() {
    let input: Vec<u8> = vec![2, 4, 8, 16];
    let output = copy_via_gpu(input);
    for val in output {
        println!("{}", val);
    }
}
