use wgpu::util::DeviceExt;

use super::{constants, window};

pub struct UniformsBindings {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    constants_buffer: wgpu::Buffer,
    window_buffer: wgpu::Buffer,
}

impl UniformsBindings {
    pub fn new(
        device: &wgpu::Device,
        constants_slice: &[constants::Constants],
        window_slice: &[window::Window],
    ) -> Self {
        // let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: None,
        //     contents: bytemuck::cast_slice(entity_slice),
        //     usage,
        // });

        //  wgpu::BufferUsages::UNIFORM,
        //               wgpu::BufferBindingType::Uniform,

        let constants_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(constants_slice),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let window_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(window_slice),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
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
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: constants_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: window_buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        Self {
            bind_group_layout,
            bind_group,
            constants_buffer,
            window_buffer,
        }
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn update_constants_buffer(
        &mut self,
        queue: &wgpu::Queue,
        constants_slice: &[constants::Constants],
    ) {
        queue.write_buffer(
            &self.constants_buffer,
            0,
            bytemuck::cast_slice(constants_slice),
        );
    }

    pub fn update_window_buffer(&mut self, queue: &wgpu::Queue, window_slice: &[window::Window]) {
        queue.write_buffer(&self.window_buffer, 0, bytemuck::cast_slice(window_slice));
    }
}
