use wgpu::util::DeviceExt;

use super::{
    bvh_node::BvhNode, cuboid::Cuboid, linear_constant_medium::LinearConstantMedium,
    linear_hittable::*, material::Material, sphere::Sphere,
};

/// The basic linearized version of the scene, each vector is separately bound to a different bind group entry in the scene layout group (due to their dynamic nature in length)
#[derive(Debug)]
pub struct LinearSceneBvh {
    pub background: Material,
    pub slice_plane: Cuboid,
    pub materials: Vec<Material>,
    pub hittables: Vec<LinearHittable>,
    pub bvh_nodes: Vec<BvhNode>,
    pub spheres: Vec<Sphere>,
    pub cuboids: Vec<Cuboid>,
    pub constant_mediums: Vec<LinearConstantMedium>,
}

impl LinearSceneBvh {
    pub fn null_index_ptr() -> u32 {
        u32::max_value()
    }

    /// Creates an empty scene
    pub fn new() -> Self {
        LinearSceneBvh {
            background: Material::empty(),
            slice_plane: Cuboid::empty(),
            materials: vec![],
            hittables: vec![],
            bvh_nodes: vec![],
            spheres: vec![],
            cuboids: vec![],
            constant_mediums: vec![],
        }
    }

    /// The WGPU binding groups must be non-empty, so place an empty/placeholder value in any empty array
    pub fn check_pad_empty_arrays(&mut self) {
        if self.materials.len() == 0 {
            panic!("Expect at least 1 material defined");
        }
        if self.hittables.len() == 0 {
            panic!("Expect at least 1 hittable defined");
        }
        if self.bvh_nodes.len() == 0 {
            self.bvh_nodes.push(BvhNode::empty());
        }
        if self.spheres.len() == 0 {
            self.spheres.push(Sphere::empty());
        }
        if self.cuboids.len() == 0 {
            self.cuboids.push(Cuboid::empty());
        }
        if self.constant_mediums.len() == 0 {
            self.constant_mediums.push(LinearConstantMedium::empty());
        }
    }

    pub fn debug_print(&self) {
        println!("LinearSceneBvh:");
        println!("background: {:?}", self.background);
        println!("materials: {:?}", self.materials);
        for hittable in self.hittables.iter() {
            if hittable.geometry_type == 0 {
                println!(
                    "\n BVH Node: {:?}",
                    self.bvh_nodes[hittable.get_scene_index()]
                );
            } else if hittable.geometry_type == 1 {
                println!("\n Sphere: {:?}", self.spheres[hittable.get_scene_index()]);
            } else if hittable.geometry_type == 2 {
                println!("\n Cuboid: {:?}", self.cuboids[hittable.get_scene_index()]);
            } else if hittable.geometry_type == 3 {
                println!(
                    "\n Constant Medium (volume): {:?}",
                    self.constant_mediums[hittable.get_scene_index()]
                );
            }
        }
    }

    pub fn create_device_buffer_binding(
        &self,
        device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        // Create bind group layout.
        let bind_group_entries: Vec<wgpu::BindGroupLayoutEntry> = (0..8)
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
        let background = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[self.background]),
            usage: buffer_usage,
        });
        let slice_plane = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[self.slice_plane]),
            usage: buffer_usage,
        });
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
        let constant_mediums = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.constant_mediums[..]),
            usage: buffer_usage,
        });

        // Finally create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: background.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: slice_plane.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: materials.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: hittables_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: bvh_nodes_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: spheres.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: cuboids.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: constant_mediums.as_entire_binding(),
                },
            ],
            label: None,
        });

        // Return data
        (bind_group_layout, bind_group)
    }
}
