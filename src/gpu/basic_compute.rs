use std::num::NonZeroU32;

use wgpu::{BindGroupLayout, BufferBinding};

use super::gpu::GPU;

pub fn basic_compute(gpu: &GPU, input: &[u8]) -> Vec<u8> {
    let input_buffer = gpu.queue_write(input, Some("Write Buffer"));
    let output_buffer = gpu.read_buffer(input.len() as u64, Some("Read Buffer"));

    let mut bindGroupLayout =
        gpu.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: Some(NonZeroU32::new(1).unwrap()),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: Some(NonZeroU32::new(1).unwrap()),
                    },
                ],
            });

    let mut bindGroup = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bindGroupLayout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &input_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &output_buffer,
                    offset: 0,
                    size: None,
                }),
            },
        ],
    });

    let mut pipeline_layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bindGroupLayout],
            push_constant_ranges: &[],
        });

    let mut shaderModule = gpu
        .device
        .create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "basic_compute.wgsl"
            ))),
        });

    let mut compute_pipeline =
        gpu.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Compute Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shaderModule,
                entry_point: "main",
            });

    let mut command_encoder = gpu.command_encoder(Some("Command Encoder"));
    {
        let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });

        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bindGroup, &[]);
        compute_pass.dispatch(4, 1, 1);
    }
    let mut compute_commands = command_encoder.finish();

    gpu.queue.submit([compute_commands]);

    return gpu.read_from(&output_buffer).to_vec();
}
