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

// x/y in [0,1]
fn camera_ray(index: u32) -> Ray {
   let x : u32 = index % dimensions.width;
   let y : u32 = index / dimensions.width;
   let view_x = f32(x) / f32(dimensions.width - 1);
   let view_y = f32(y) / f32(dimensions.height - 1);
   let y_len : f32 = atan(camera.vertical_fov / 2);
   let x_len : f32 = camera.aspect_ratio * y_len;
   let dir_x : f32 = 2.0 * (view_x - 0.5) * x_len;
   let dir_y : f32 = 2.0 * (view_y - 0.5) * y_len;
   let dir_z : f32 = 1.0;
   let dir : vec3<f32> = normalize(vec3(dir_x, dir_y, dir_z));
   return Ray(vec3(camera.x,camera.y,camera.z), dir);
}

fn ray_collision_times(ray: Ray, closest_point: vec3<f32>, sphere: Sphere) -> vec2<f32> {
    let rad_sq : f32 = sphere.radius * sphere.radius;
    let dist_sq :f32 = pow(length(closest_point), 2);
    let dist_back :f32 = sqrt(rad_sq - dist_sq);
    let early_collision : vec3<f32> = closest_point + sphere.pos - dist_back * ray.dir;
    let early_collision_time = dot(early_collision - ray.pos, ray.dir);
    return  vec2<f32>(early_collision_time, early_collision_time+2*dist_back);
}

fn sphere_normal(sphere: Sphere, ray:Ray, collision_time:f32) -> vec3<f32> {
    return normalize(ray.dir * collision_time - sphere.pos);
}


fn ray_collides(ray: Ray) -> vec4<f32> {
    var collision = vec4<f32>(0.0, 0.0, 0.0, 20000000.0);
    for (var i : u32 = 0; i < arrayLength(&world.spheres); i++) {
        let sphere : Sphere = world.spheres[i];
        let p : vec3<f32> = (sphere.pos - ray.pos);
        let closest_point : vec3<f32> = dot(ray.dir, p) * ray.dir - p;
        let closest_dist : f32 = length(closest_point);
        if (closest_dist <= sphere.radius) {
            let collision_times : vec2<f32> = ray_collision_times(ray, closest_point, sphere);
            if (collision_times.x > 0.0 && collision_times.x < collision.w) {
                let normal : vec3<f32> = sphere_normal(sphere, ray, collision_times.x);
                collision = vec4<f32>(normal, collision_times.x);
            } else if (collision_times.y > 0.0 && collision_times.y < collision.w) {
                let normal : vec3<f32> = sphere_normal(sphere, ray, collision_times.y);
                collision = vec4<f32>(normal, collision_times.y);
            }
        }
    }
    return collision;
}

fn to_color_int(val: f32, offset: u32) -> u32 {
    return (u32(clamp(255.0 * val, 0.0, 255.0)) & 255u) << offset;
}

fn to_color(color: vec3<f32>) -> u32 {
    return to_color_int(color.x, 24u) |  to_color_int(color.y, 16u) | to_color_int(color.z, 8u) | 255u;
}

let LIGHT_BLUE : vec3<f32> = vec3<f32>(0.5, 0.7, 1.0);
let WHITE : vec3<f32> = vec3<f32>(1, 1, 1);

fn sky_color(ray_dir: vec3<f32>) -> vec3<f32> {
    return mix(LIGHT_BLUE, WHITE, vec3<f32>(ray_dir.y, ray_dir.y, ray_dir.y));
}

fn ray_color(ray: Ray) -> vec3<f32> {
    let color : vec3<f32> = vec3<f32>(1,1,1);
    for (var collision: u32 = 0; collision < 10; collision++) {
        let ray_collision = ray_collides(ray);
        if (ray_collision.w >= 10000000.0) {
            return color * sky_color(ray.dir);
        }
        return (ray_collision.xyz + vec3<f32>(1,1,1)) / 2;
    }
    return vec3<f32>(0,0,0);
}

fn pixel_color(index: u32) -> vec3<f32> {
    let ray = camera_ray(index);
    return ray_color(ray);
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
            output.colors[index].rgba = to_color(pixel_color(index));
    }
}