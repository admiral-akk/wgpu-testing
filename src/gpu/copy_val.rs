use wgpu::BufferDescriptor;

use super::gpu::GPU;

pub fn copy_val(gpu: &GPU, input: &Vec<u8>) -> Vec<u8> {
    let mut gpu_write_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Write Buffer"),
        size: input.len() as u64,
        usage: wgpu::BufferUsages::MAP_WRITE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut gpu_read_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Read Buffer"),
        size: input.len() as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    gpu.queue.write_buffer(&gpu_write_buffer, 0, &input);
    let mut copy_encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Copy Encoder"),
        });
    copy_encoder.copy_buffer_to_buffer(&gpu_write_buffer, 0, &gpu_read_buffer, 0, 4);
    let copy_commands = copy_encoder.finish();

    gpu.queue.submit([copy_commands]);

    let read_slice = gpu_read_buffer.slice(..);

    let mapping = read_slice.map_async(wgpu::MapMode::Read);
    gpu.device.poll(wgpu::Maintain::Wait);
    pollster::block_on(mapping).unwrap();

    return read_slice.get_mapped_range().to_vec();
}
