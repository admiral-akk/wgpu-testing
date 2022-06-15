pub struct GPU {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GPU {
    pub async fn new() -> GPU {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // The adapter is our interface to the GPU
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: true,
            compatible_surface: None,
        }))
        .unwrap();

        // The device creates compute/rendering resourses
        // The queue exectures CommandBuffers
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await
            .unwrap();
        GPU {
            adapter,
            device,
            queue,
        }
    }

    pub fn queue_write(&self, input: &[u8], label: Option<&str>) -> wgpu::Buffer {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: input.len() as u64,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&buffer, 0, &input);
        return buffer;
    }

    pub fn read_buffer(&self, len: u64, label: Option<&str>) -> wgpu::Buffer {
        return self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: len,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    }

    pub fn command_encoder(&self, label: Option<&str>) -> wgpu::CommandEncoder {
        return self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: label });
    }

    pub fn read_from<'a>(&self, read_buffer: &'a wgpu::Buffer) -> wgpu::BufferView<'a> {
        let read_slice = read_buffer.slice(..);
        let mapping = read_slice.map_async(wgpu::MapMode::Read);
        self.device.poll(wgpu::Maintain::Wait);
        pollster::block_on(mapping).unwrap();
        return read_slice.get_mapped_range();
    }
}
