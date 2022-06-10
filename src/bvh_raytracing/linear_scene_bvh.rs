use wgpu::util::DeviceExt;

use super::{
    bvh_node::BvhNode, cuboid::Cuboid, linear_hittable::*, material::Material, sphere::Sphere,
};

///
/// The basic linearized version of the scene, each vector is separately bound to a different bind group entry in the scene layout group (due to their dynamic nature in length)
///
#[derive(Debug)]
pub struct LinearSceneBvh {
    pub materials: Vec<Material>,
    /// TODO - this is being refactored to contain a redirection the the index of the geometry type in the appropriate linear scene array for that type.
    pub hittables: Vec<LinearHittable>,
    pub bvh_nodes: Vec<BvhNode>,
    pub spheres: Vec<Sphere>,
    pub cuboids: Vec<Cuboid>,
}

impl LinearSceneBvh {
    pub fn create_device_buffer_binding(
        &self,
        device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        // Create bind group layout.
        let bind_group_entries: Vec<wgpu::BindGroupLayoutEntry> = (0..5)
            .map(|i| wgpu::BindGroupLayoutEntry {
                binding: i,
                count: None,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: (true) },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            })
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &bind_group_entries[..],
        });

        // Create buffers
        let buffer_usage = wgpu::BufferUsages::STORAGE;
        let materials = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.materials[..]),
            usage: buffer_usage,
        });
        let hittables_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.hittables[..]),
            usage: buffer_usage,
        });
        let bvh_nodes_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.bvh_nodes[..]),
            usage: buffer_usage,
        });
        let spheres = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.spheres[..]),
            usage: buffer_usage,
        });
        let cuboids = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.cuboids[..]),
            usage: buffer_usage,
        });

        // Finally create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: materials.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: hittables_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: bvh_nodes_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: spheres.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: cuboids.as_entire_binding(),
                },
            ],
            label: None,
        });

        // Return data
        (bind_group_layout, bind_group)
    }
}
