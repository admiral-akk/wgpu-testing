mod color;
mod dimensions;
mod image_writer;

use wgpu_testing::{init_gpu_compute_shader, write_test_image};

fn main() {
    let input: Vec<u8> = vec![2, 4, 8, 16];
    let output = init_gpu_compute_shader(&input);
    for val in output {
        println!("{}", val);
    }
}
