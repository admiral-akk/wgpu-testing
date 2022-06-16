use wgpu_wasm::{structs::dimensions::Dimensions, write_test_image, write_test_image_via_gpu};

fn main() {
    let dimensions = Dimensions::new(300, 200);
    pollster::block_on(write_test_image_via_gpu(&dimensions));
    write_test_image(&dimensions);
}
