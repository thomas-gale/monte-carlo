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
        let scene_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene Buffer"),
            contents: bytemuck::cast_slice(&self.spheres[..]),
            usage: wgpu::BufferUsages::STORAGE
        });

        // SceneBuffer {
        // 	buffer: scene__buffer,

        // }

        scene_buffer
    }
}
