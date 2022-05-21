use super::sphere::Sphere;
use wgpu::util::DeviceExt;

pub struct Scene {
    spheres: Vec<Sphere>,
}

// pub struct SceneBuffer {
// 	  pub buffer: wgpu::Buffer,
// 		pub bind_group: wgpu::BindGroup,
// }

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![
                Sphere {
                    center: [0.0, 0.0, 1.0],
                    radius: 0.25,
                },
                Sphere {
                    center: [0.0, -100.5, 1.0],
                    radius: 100.0,
                },
            ],
        }
    }

    pub fn create_scene_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene Storage Buffer"),
            contents: bytemuck::cast_slice(&self.spheres[..]),
            usage: wgpu::BufferUsages::STORAGE,
        })
    }

    // TODO - passed the buffer back into this function seems clunky
    pub fn create_binding(
        &self,
        buffer: &wgpu::Buffer,
        device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });
        (bind_group_layout, bind_group)
    }
}
