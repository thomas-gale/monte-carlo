use wgpu::util::DeviceExt;

use super::window;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ResultUniforms {
    pub pass_index: u32,
}

pub struct Result {
    texture: wgpu::Texture,
    uniforms: ResultUniforms,
    uniforms_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl Result {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, window: window::Window) -> Self {
        // Initialize the result texture (where the accumulated (average) sampled pixel colors will be stored frame to frame)
        let texture_size = wgpu::Extent3d {
            width: window.width_pixels,
            height: window.height_pixels,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING,
            label: None,
        });

        let inital_data: Vec<u8> =
            vec![0; texture_size.width as usize * texture_size.height as usize * 4 * 4];

        Self::update_texture(device, queue, &texture, &inital_data[..], texture_size);

        // Initialize the uniforms buffer (to keep track of things like pass index)
        let uniforms = ResultUniforms { pass_index: 0 };

        let uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create a combined bind group layout and bind group for the result data.
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: wgpu::TextureFormat::Rgba32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
            ],
            label: None,
        });

        // Generate texture view for the binding resource.
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: uniforms_buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        Result {
            // texture_size,
            texture,
            bind_group_layout,
            bind_group,
            uniforms,
            uniforms_buffer,
        }
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn increment_pass_index(&mut self, queue: &wgpu::Queue) {
        self.set_pass_index(queue, self.uniforms.pass_index + 1);
    }

    pub fn reset_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: winit::dpi::PhysicalSize<u32>,
    ) {
        let texture_size = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        };

        let inital_data: Vec<u8> =
            vec![0; texture_size.width as usize * texture_size.height as usize * 4 * 4];

        self.set_pass_index(queue, 0);
        Self::update_texture(device, queue, &self.texture, &inital_data[..], texture_size);
    }

    fn set_pass_index(&mut self, queue: &wgpu::Queue, index: u32) {
        self.uniforms.pass_index = index;
        queue.write_buffer(
            &self.uniforms_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    fn update_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &wgpu::Texture,
        contents: &[u8],
        size: wgpu::Extent3d,
    ) {
        let source_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents,
            usage: wgpu::BufferUsages::COPY_SRC,
        });

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &source_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(size.width * 4 * 4),
                    rows_per_image: std::num::NonZeroU32::new(size.height),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        queue.submit(std::iter::once(encoder.finish()));
    }
}
