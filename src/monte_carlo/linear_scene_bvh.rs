use cgmath::Matrix4;
use wgpu::util::DeviceExt;

use super::{
    bvh_node::BvhNode,
    construction_scene::recompute_bvh,
    cuboid::Cuboid,
    linear_constant_medium::LinearConstantMedium,
    linear_hittable::*,
    material::Material,
    sphere::Sphere,
    triangle::{Triangle, TriangleVertex},
};

/// The basic linearized version of the scene, each vector is separately bound to a different bind group entry in the scene layout group (due to their dynamic nature in length)
#[derive(Debug)]
pub struct LinearSceneBvh {
    pub materials: Vec<Material>,
    pub hittables: Vec<LinearHittable>,
    pub bvh_nodes: Vec<BvhNode>,
    pub spheres: Vec<Sphere>,
    pub cuboids: Vec<Cuboid>,
    pub constant_mediums: Vec<LinearConstantMedium>,
    pub tri_verts: Vec<TriangleVertex>,
    pub tris: Vec<Triangle>,

    pub materials_buffer: Option<wgpu::Buffer>,
    pub hittables_buffer: Option<wgpu::Buffer>,
    pub bvh_nodes_buffer: Option<wgpu::Buffer>,
    pub spheres_buffer: Option<wgpu::Buffer>,
    pub cuboids_buffer: Option<wgpu::Buffer>,
    pub constant_mediums_buffer: Option<wgpu::Buffer>,
    pub mesh_tri_verts_buffer: Option<wgpu::Buffer>,
    pub mesh_tris_buffer: Option<wgpu::Buffer>,
}

impl LinearSceneBvh {
    pub fn null_index_ptr() -> u32 {
        u32::max_value()
    }

    /// Creates an empty scene
    /// TODO refactor away this 'partially' constructed object workflow - it's bug prone.
    pub fn new() -> Self {
        LinearSceneBvh {
            materials: vec![],
            hittables: vec![],
            bvh_nodes: vec![],
            spheres: vec![],
            cuboids: vec![],
            constant_mediums: vec![],
            tri_verts: vec![],
            tris: vec![],

            materials_buffer: None,
            hittables_buffer: None,
            bvh_nodes_buffer: None,
            spheres_buffer: None,
            cuboids_buffer: None,
            constant_mediums_buffer: None,
            mesh_tri_verts_buffer: None,
            mesh_tris_buffer: None,
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
        if self.tri_verts.len() == 0 {
            self.tri_verts.push(TriangleVertex::empty());
        }
        if self.tris.len() == 0 {
            self.tris.push(Triangle::empty());
        }
    }

    pub fn debug_print(&self) {
        println!("LinearSceneBvh:");
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
            } else if hittable.geometry_type == 4 {
                println!("\n Triangle: {:?}", self.tris[hittable.get_scene_index()]);
            }
        }
    }

    /// Updates the buffer inside the struct and returns binding information
    pub fn create_device_buffers(
        &mut self,
        device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        // Create bind group layout. (This (8) is the maximum number of bindings for a group)
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
        let buffer_usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST;
        let materials_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
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
        let spheres_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.spheres[..]),
            usage: buffer_usage,
        });
        let cuboids_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.cuboids[..]),
            usage: buffer_usage,
        });
        let constant_mediums_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.constant_mediums[..]),
                usage: buffer_usage,
            });
        let triangle_vertices_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.tri_verts[..]),
                usage: buffer_usage,
            });
        let triangles_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.tris[..]),
            usage: buffer_usage,
        });

        // Finally create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: materials_buffer.as_entire_binding(),
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
                    resource: spheres_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: cuboids_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: constant_mediums_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: triangle_vertices_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: triangles_buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        // Assign internal buffers
        self.materials_buffer = Some(materials_buffer);
        self.hittables_buffer = Some(hittables_buffer);
        self.bvh_nodes_buffer = Some(bvh_nodes_buffer);
        self.spheres_buffer = Some(spheres_buffer);
        self.cuboids_buffer = Some(cuboids_buffer);
        self.constant_mediums_buffer = Some(constant_mediums_buffer);
        self.mesh_tri_verts_buffer = Some(triangle_vertices_buffer);
        self.mesh_tris_buffer = Some(triangles_buffer);

        // Return data
        (bind_group_layout, bind_group)
    }

    pub fn update_buffers(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.materials_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.materials[..]),
        );
        queue.write_buffer(
            &self.hittables_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.hittables[..]),
        );
        queue.write_buffer(
            &self.bvh_nodes_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.bvh_nodes[..]),
        );
        queue.write_buffer(
            &self.spheres_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.spheres[..]),
        );
        queue.write_buffer(
            &self.cuboids_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.cuboids[..]),
        );
        queue.write_buffer(
            &self.constant_mediums_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.constant_mediums[..]),
        );
        queue.write_buffer(
            &self.mesh_tri_verts_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.tri_verts[..]),
        );
        queue.write_buffer(
            &self.mesh_tris_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&self.tris[..]),
        );
    }

    /// Helper function to update a hittable in the scenes
    /// This will internally recompute the bvh and update all scene data in buffers that are bound to GPU
    pub fn transform_hittable_by(
        &mut self,
        queue: &wgpu::Queue,
        hittable: &LinearHittable,
        transform: Matrix4<f32>,
    ) {
        // Request the referenced hittable to update the underlying data in the scene.
        hittable.transform_by(self, transform);

        // Recompute the BVH
        recompute_bvh(self);

        // Push changes to device
        self.update_buffers(queue);
    }
}
