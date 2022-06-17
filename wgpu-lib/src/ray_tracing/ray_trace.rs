use crate::{
    gpu::gpu::GPU,
    ray_tracing::structs::camera::Camera,
    structs::{color::Color, dimensions::Dimensions},
};

use super::structs::{sphere::Sphere, vec3::Vec3};

pub async fn ray_trace(gpu: &GPU, dimensions: &Dimensions) -> Vec<Color> {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        dimensions,
        20.0,
    );

    let mut world = Vec::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 3.0), 1.0));

    let dimensions_buffer = gpu.write_buffer_init_struct(dimensions, Some("Dimensions"));
    let camera_buffer = gpu.write_buffer_init_struct(&camera, Some("Camera"));
    let world = gpu.write_buffer_init_array(&world, Some("World"));
    let colors_size = (std::mem::size_of::<Color>() * dimensions.size()) as u64;
    let colors = gpu.read_buffer(colors_size, Some("Color Output Buffer"));
    let read_buffer = gpu.staging_buffer(colors_size, Some("Read Buffer"));

    let bind_group_layout = gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                gpu.bind_group_layout_entry(0, false, true, false),
                gpu.bind_group_layout_entry(1, false, false, false),
                gpu.bind_group_layout_entry(2, true, true, true),
                gpu.bind_group_layout_entry(3, true, false, true),
            ],
        });

    let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            gpu.bind_group_entry(0, &dimensions_buffer),
            gpu.bind_group_entry(1, &camera_buffer),
            gpu.bind_group_entry(2, &world),
            gpu.bind_group_entry(3, &colors),
        ],
    });
    let pipeline_layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

    let shader_module = gpu.shader_module(include_str!("ray_trace.wgsl"), Some("Shader Module"));

    let compute_pipeline =
        gpu.compute_pipeline(&pipeline_layout, &shader_module, Some("Compute Pipeline"));

    let group_count = 128;

    let mut command_encoder = gpu.command_encoder(Some("Command Encoder"));
    {
        let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });

        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch(group_count, 1, 1);
    }
    command_encoder.copy_buffer_to_buffer(&colors, 0, &read_buffer, 0, colors_size);
    let compute_commands = command_encoder.finish();

    gpu.queue.submit([compute_commands]);

    return gpu.read_from(&read_buffer).await.to_vec();
}
