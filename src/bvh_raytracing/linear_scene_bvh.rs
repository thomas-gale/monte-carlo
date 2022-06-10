use wgpu::util::DeviceExt;

use super::{
    bvh_node::BvhNode, construction_scene_bvh_node::SceneBvhConstructionNode, cuboid::Cuboid,
    hittable_primitive::HittablePrimitive, linear_hittable::*, material::Material, sphere::Sphere,
};

///
/// The basic linearized version of the scene, ready to be transferred to the GPU
///
#[derive(Debug)]
pub struct LinearSceneBvh {
    /// TODO - this is being refactored to contain a redirection the the index of the geometry type in the appropriate linear scene array for that type.
    hittables: Vec<LinearHittable>,
    bvh_nodes: Vec<BvhNode>,
    spheres: Vec<Sphere>,
    cuboids: Vec<Cuboid>,
    materials: Vec<Material>,
}

impl LinearSceneBvh {
    /// TODO - this is being replaced with direct construction from the "Construction Scene" methods
    /// Experimental function to build a BVH from a slice of spheres
    pub fn build_from_spheres(spheres: &[Sphere]) -> Self {
        let hittables: Vec<LinearHittable> = spheres
            .iter()
            .map(|sphere| LinearHittable::new(HittablePrimitive::Sphere(*sphere)))
            .collect();

        let bvh_construction = SceneBvhConstructionNode::new(&hittables[..]);
        bvh_construction.flatten()
    }

    /// TODO - this is being replaced with direct construction from the "Construction Scene" methods
    pub fn build_from_hittables(hittables: Vec<LinearHittable>) -> Self {
        LinearSceneBvh {
            hittables,
            bvh_nodes: vec![BvhNode::empty()],
            spheres: vec![Sphere::empty()],
            cuboids: vec![Cuboid::empty()],
            materials: vec![Material::empty()],
        }
    }

    /// TODO - this is being replaced with direct construction from the "Construction Scene" methods
    pub fn get_hittables(&self) -> &Vec<LinearHittable> {
        &self.hittables
    }

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
        let materials = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.materials[..]),
            usage: buffer_usage,
        });

        // Finally create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: hittables_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: bvh_nodes_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: spheres.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: cuboids.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: materials.as_entire_binding(),
                },
            ],
            label: None,
        });

        // Return data
        (bind_group_layout, bind_group)
    }
}
