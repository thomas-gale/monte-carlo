use wgpu::util::DeviceExt;

use super::window;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ResultUniforms {
    pub pass_index: u32,
}

pub struct Result {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniforms: ResultUniforms,
    uniforms_buffer: wgpu::Buffer,
}

impl Result {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, window: window::Window) -> Self {
        let inital_data: Vec<u8> =
            vec![0; window.width_pixels as usize * window.height_pixels as usize * 4* 4];

        let size = wgpu::Extent3d {
            width: window.width_pixels,
            height: window.height_pixels,
            depth_or_array_layers: 1,
        };

        println!("{:?}", size);
        println!("{:?}", window);

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING,
            label: None,
        });

        // queue.write_texture(
        //     wgpu::ImageCopyTexture {
        //         texture: &texture,
        //         mip_level: 0,
        //         origin: wgpu::Origin3d::ZERO,
        //         aspect: wgpu::TextureAspect::All,
        //     },
        //     &inital_data[..],
        //     wgpu::ImageDataLayout {
        //         offset: 0,
        //         bytes_per_row: std::num::NonZeroU32::new(window.width_pixels * 4),
        //         rows_per_image: std::num::NonZeroU32::new(window.height_pixels),
        //     },
        //     size,
        // );

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &inital_data[..],
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        });

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // NOTES:
        // 1. try setting winit image dimensions to multiple of 256 to test
        // 2. inflate the texture size to nearest multiple of 256 above
        // 2a. pass the actual texture size as part of this binding group to the shader
        // 2b. related side note: investigate invocation number / passing in increasing index to the buffer on each redraw.
        // 2c. update the shader code to address the part of the texture that actually contains data - should have a 1:1 correspondance from the fragment position the texture data.
        // 2d. in terms of pixel averaging - use the index * number of samples each invocation to track the average (using a mean average)
        //   e.g. av_col = sample_col / N + (av_col * (N - 1)) / N

        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout { offset: 0, bytes_per_row: std::num::NonZeroU32::new(window.width_pixels * 4 * 4), rows_per_image: std::num::NonZeroU32::new(window.height_pixels) }
                // offset: 0,
                // bytes_per_row: 4 * window.width_pixels,
                // rows_per_image: window.height_pixels,
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                // array_layer: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        queue.submit(std::iter::once(encoder.finish()));

        // let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        // 	label: None,
        // 	contents:

        // })

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        //     address_mode_u: wgpu::AddressMode::ClampToEdge,
        //     address_mode_v: wgpu::AddressMode::ClampToEdge,
        //     address_mode_w: wgpu::AddressMode::ClampToEdge,
        //     mag_filter: wgpu::FilterMode::Nearest,
        //     min_filter: wgpu::FilterMode::Nearest,
        //     mipmap_filter: wgpu::FilterMode::Nearest,
        //     ..Default::default()
        // });

        let uniforms = ResultUniforms { pass_index: 0 };

        let uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: wgpu::TextureFormat::Rgba32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        // sample_type: wgpu::TextureSampleType::Float { filterable: true },
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
                // wgpu::BindGroupEntry {
                //     binding: 1,
                //     resource: wgpu::BindingResource::Sampler(&sampler),
                // },
            ],
            label: None,
        });

        Result {
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

    pub fn get_pass_index(&self) -> u32 {
        self.uniforms.pass_index
    }

    pub fn increment_result_index(&mut self, queue: &wgpu::Queue) {
        self.uniforms.pass_index += 1;
        queue.write_buffer(
            &self.uniforms_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
}
