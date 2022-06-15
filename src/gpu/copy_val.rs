use super::gpu::GPU;

pub fn copy_val(gpu: &GPU, input: &[u8]) -> Vec<u8> {
    let input_buffer = gpu.queue_write(input, Some("Write Buffer"));
    let output_buffer = gpu.read_buffer(input.len() as u64, Some("Read Buffer"));

    let mut copy_encoder = gpu.command_encoder(Some("Copy Encoder"));
    copy_encoder.copy_buffer_to_buffer(&input_buffer, 0, &output_buffer, 0, 4);
    let copy_commands = copy_encoder.finish();

    gpu.queue.submit([copy_commands]);

    return gpu.read_from(&output_buffer).to_vec();
}
