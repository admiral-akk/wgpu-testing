use std::num::NonZeroU32;

use wgpu::{BindGroupLayoutEntry, Buffer, ShaderModule};

pub struct GPU {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GPU {
    pub async fn new() -> GPU {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // The adapter is our interface to the GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
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

    fn queue_write<T: bytemuck::Pod>(&self, input: &[T], buffer: &wgpu::Buffer) {
        self.queue
            .write_buffer(&buffer, 0, &bytemuck::cast_slice(&input));
    }

    pub fn write_buffer_init_array<T: bytemuck::Pod>(
        &self,
        input: &[T],
        label: Option<&str>,
    ) -> wgpu::Buffer {
        let bytes: &[u8] = bytemuck::cast_slice(input);
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: bytes.len() as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        self.queue_write(bytes, &buffer);
        return buffer;
    }

    pub fn write_buffer_init_struct<T: bytemuck::Pod>(
        &self,
        input: &T,
        label: Option<&str>,
    ) -> wgpu::Buffer {
        let bytes: &[u8] = bytemuck::bytes_of(input);
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: bytes.len() as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        self.queue_write(bytes, &buffer);
        return buffer;
    }

    pub fn read_buffer(&self, len: u64, label: Option<&str>) -> wgpu::Buffer {
        return self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: len,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    }

    pub fn staging_buffer(&self, len: u64, label: Option<&str>) -> wgpu::Buffer {
        return self.device.create_buffer(&wgpu::BufferDescriptor {
            label: label,
            size: len,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    }

    pub fn command_encoder(&self, label: Option<&str>) -> wgpu::CommandEncoder {
        return self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: label });
    }

    pub fn shader_module(&self, wgsl_code: &str, label: Option<&str>) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: label,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(wgsl_code)),
            })
    }

    pub fn bind_group_layout_entry(
        &self,
        binding: u32,
        read_only: bool,
        is_array: bool,
    ) -> BindGroupLayoutEntry {
        let mut count: Option<NonZeroU32> = None;
        if is_array {
            count = Some(NonZeroU32::new(1).unwrap());
        }
        wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage {
                    read_only: read_only,
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: count,
        }
    }
    pub fn bind_group_entry<'a>(
        &self,
        binding: u32,
        buffer: &'a Buffer,
    ) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }
    }
    pub fn compute_pipeline(
        &self,
        pipeline_layout: &wgpu::PipelineLayout,
        shader_module: &ShaderModule,
        label: Option<&str>,
    ) -> wgpu::ComputePipeline {
        self.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: label,
                layout: Some(pipeline_layout),
                module: shader_module,
                entry_point: "main",
            })
    }

    pub async fn read_from<T: bytemuck::Pod>(&self, read_buffer: &wgpu::Buffer) -> Vec<T> {
        let read_slice = read_buffer.slice(..);
        let mapping = read_slice.map_async(wgpu::MapMode::Read);
        self.device.poll(wgpu::Maintain::Wait);
        mapping.await.unwrap();
        return bytemuck::cast_slice(&read_slice.get_mapped_range()).to_vec();
    }
}
