pub struct GPU {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GPU {
    pub async fn new() -> GPU {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        for x in instance.enumerate_adapters(wgpu::Backends::all()) {
            println!("{}", x.get_info().name);
        }

        // The adapter is our interface to the GPU
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        }))
        .unwrap();

        // The device creates compute/rendering resourses
        // The queue exectures CommandBuffers
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: wgpu::Features::BUFFER_BINDING_ARRAY
                        | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY,
                    limits: wgpu::Limits::default(),
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

    pub fn queue_write<T: bytemuck::Pod>(&self, input: &[T], label: Option<&str>) -> wgpu::Buffer {
        let bytes: &[u8] = bytemuck::cast_slice(&input);
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: bytes.len() as u64,
            usage: wgpu::BufferUsages::all(),
            mapped_at_creation: false,
        });
        self.queue
            .write_buffer(&buffer, 0, &bytemuck::cast_slice(&bytes));
        return buffer;
    }

    pub fn read_buffer(&self, len: u64, label: Option<&str>) -> wgpu::Buffer {
        return self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: len,
            usage: wgpu::BufferUsages::all(),
            mapped_at_creation: false,
        });
    }

    pub fn command_encoder(&self, label: Option<&str>) -> wgpu::CommandEncoder {
        return self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: label });
    }

    pub fn read_from<T: bytemuck::Pod>(&self, read_buffer: &wgpu::Buffer) -> Vec<T> {
        let read_slice = read_buffer.slice(..);
        let mapping = read_slice.map_async(wgpu::MapMode::Read);
        self.device.poll(wgpu::Maintain::Wait);
        pollster::block_on(mapping).unwrap();
        return bytemuck::cast_slice(&read_slice.get_mapped_range().to_vec()).to_vec();
    }
}
