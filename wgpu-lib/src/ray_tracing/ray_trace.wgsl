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

struct Sphere {
    pos: vec3<f32>,
    radius: f32,
}

struct World {
    spheres: array<Sphere>
}

struct Camera {
    x: f32,
    y: f32,
    z: f32,
    dir_x: f32,
    dir_y: f32,
    dir_z: f32,
    aspect_ratio: f32,
    vertical_fov: f32,
}

struct Ray {
    pos: vec3<f32>,
    dir: vec3<f32>,
}

@group(0) @binding(0) var<uniform> dimensions : Dimensions;
@group(0) @binding(1) var<uniform> camera : Camera;
@group(0) @binding(2) var<storage, read> world : World;
@group(0) @binding(3) var<storage, read_write> output : Colors;

// x/y in [-1,1]
fn camera_ray(x:f32, y:f32) -> Ray {
   return Ray(vec3(camera.x,camera.y,camera.z), vec3(camera.dir_x,camera.dir_y,camera.dir_z));
}

@compute
@workgroup_size(128,1,1)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    // Guard against out-of-bounds work group sizes
    if (global_id.x >= arrayLength(&output.colors)) {
    return;
    }
    let work_size : u32 = arrayLength(&output.colors) / 128u;
    for (var index : u32 = global_id.x * work_size; index < (global_id.x + 1u) * work_size; index = index + 1u) {
        let x : u32 = index % dimensions.width;
        let y : u32 = index / dimensions.width;
        let ray = camera_ray(f32(x),f32(y));
        let r : u32 = ((((255u * x) / dimensions.width) & 255u) << 24u);
        let g : u32 = ((((255u * y) / dimensions.height) & 255u) << 16u);
        output.colors[index].rgba = r | g | 255;
    }
}