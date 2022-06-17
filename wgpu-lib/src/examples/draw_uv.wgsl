struct Color {
  rgba: u32,
};

struct Colors {
    colors : array<Color>,
};
struct Dimensions {
    width: u32,
    height: u32,
};

@group(0) @binding(0) var<storage, read> dimensions : Dimensions;
@group(0) @binding(1) var<storage, read_write> output : Colors;
@compute
@workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    // Guard against out-of-bounds work group sizes
    if (global_id.x >= arrayLength(&output.colors)) {
    return;
    }
    let work_size : u32 = arrayLength(&output.colors) / 128u;
    for (var index : u32 = global_id.x * work_size; index < (global_id.x + 1u) * work_size; index = index + 1u) {
        let x : u32 = index % dimensions.width;
        let y : u32 = index / dimensions.width;
        let r : u32 = ((((255u * x) / dimensions.width) & 255u) << 24u);
        let g : u32 = ((((255u * y) / dimensions.height) & 255u) << 16u);
        output.colors[index].rgba = r | g | 255;
    }
}