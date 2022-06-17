use crate::gpu::gpu::GPU;

pub async fn copy_val<T: bytemuck::Pod>(gpu: &GPU, input: &[T]) -> Vec<u32> {
    let input_size = (std::mem::size_of::<T>() as u64) * (input.len() as u64);
    let input_buffer = gpu.write_buffer_init_array(input, Some("Write Buffer"));
    let output_buffer = gpu.read_buffer(input_size, Some("Read Buffer"));

    let mut copy_encoder = gpu.command_encoder(Some("Copy Encoder"));
    copy_encoder.copy_buffer_to_buffer(&input_buffer, 0, &output_buffer, 0, input_size);
    let copy_commands = copy_encoder.finish();

    gpu.queue.submit([copy_commands]);

    return gpu.read_from(&output_buffer).await.to_vec();
}
