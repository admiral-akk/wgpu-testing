use crate::{
    color::Color,
    dimensions::{self, Dimensions},
};

use super::gpu::GPU;

pub fn draw_uv(gpu: &GPU, dimensions: &Dimensions) -> Vec<Color> {
    let input_buffer = gpu.write_buffer_init_struct(dimensions, Some("Dimensions"));
    let output_size = std::mem::size_of::<Color>() * dimensions.size();
    let output_buffer = gpu.read_buffer(output_size as u64, Some("Color Output Buffer"));

    let bind_group_layout = gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                gpu.bind_group_layout_entry(0, true, false),
                gpu.bind_group_layout_entry(1, false, false),
            ],
        });

    let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            gpu.bind_group_entry(0, &input_buffer),
            gpu.bind_group_entry(1, &output_buffer),
        ],
    });
    let pipeline_layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

    let shader_module = gpu.shader_module(include_str!("draw_uv.wgsl"), Some("Shader Module"));

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
    let compute_commands = command_encoder.finish();

    gpu.queue.submit([compute_commands]);

    return gpu.read_from(&output_buffer).to_vec();
}
