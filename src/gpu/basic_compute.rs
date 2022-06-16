use super::gpu::GPU;

pub fn basic_compute(gpu: &GPU, input: &[u32]) -> Vec<u32> {
    let input_buffer = gpu.write_buffer_init(input, Some("Write Buffer"));
    let input_size = (std::mem::size_of::<u32>() as u64) * (input.len() as u64);
    let output_buffer = gpu.read_buffer(input_size, Some("Read Buffer"));

    let bind_group_layout = gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                gpu.bind_group_layout_entry(0, true, true),
                gpu.bind_group_layout_entry(1, false, true),
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

    let shader_module =
        gpu.shader_module(include_str!("basic_compute.wgsl"), Some("Shader Module"));

    let compute_pipeline =
        gpu.compute_pipeline(&pipeline_layout, &shader_module, Some("Compute Pipeline"));

    let mut command_encoder = gpu.command_encoder(Some("Command Encoder"));
    {
        let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });

        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch(input_size as u32, 1, 1);
    }
    let compute_commands = command_encoder.finish();

    gpu.queue.submit([compute_commands]);

    return gpu.read_from(&output_buffer).to_vec();
}
