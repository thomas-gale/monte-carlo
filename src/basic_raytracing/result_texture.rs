use wgpu::util::DeviceExt;

use super::window;

pub struct ResultTexture {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl ResultTexture {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, window: window::Window) -> Self {
        let inital_data: Vec<u8> =
            vec![0; window.width_pixels as usize * window.height_pixels as usize * 4];

        let size = wgpu::Extent3d {
            width: window.width_pixels,
            height: window.height_pixels,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Uint,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING,
            label: None,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &inital_data[..],
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(window.width_pixels * 4),
                rows_per_image: std::num::NonZeroU32::new(window.height_pixels),
            },
            size,
        );

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

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: wgpu::TextureFormat::Rgba8Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        // sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // wgpu::BindGroupLayoutEntry {
                //     binding: 1,
                //     visibility: wgpu::ShaderStages::FRAGMENT,
                //     ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                //     count: None,
                // },
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
                // wgpu::BindGroupEntry {
                //     binding: 1,
                //     resource: wgpu::BindingResource::Sampler(&sampler),
                // },
            ],
            label: None,
        });

        ResultTexture {
            bind_group_layout,
            bind_group,
        }
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
