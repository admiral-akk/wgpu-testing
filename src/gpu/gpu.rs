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
}
