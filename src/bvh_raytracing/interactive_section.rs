use cgmath::{Matrix4, Vector2, Vector3};
use wgpu::util::DeviceExt;

use super::{buffer_bindings, cuboid::Cuboid, linear_scene_bvh::LinearSceneBvh, result};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct InteractiveSectionRaw(Cuboid);

pub struct InteractiveSection {
    transform: Matrix4<f32>,
    // bind_group_layout: wgpu::BindGroupLayout,
    // bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl InteractiveSection {
    pub fn new(device: &wgpu::Device, transform: Matrix4<f32>, buffer: wgpu::Buffer) -> Self {
        let raw = InteractiveSectionRaw {
            0: Cuboid::new(transform.clone(), LinearSceneBvh::null_index_ptr()),
        };

        // let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: None,
        //     contents: bytemuck::cast_slice(&[raw]),
        //     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        // });

        // let (bind_group_layout, bind_group, buffer) = buffer_bindings::create_device_buffer_binding(
        //     &[raw],
        //     &device,
        //     wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        //     wgpu::BufferBindingType::Uniform,
        // );

        InteractiveSection {
            transform,
            // bind_group_layout,
            // bind_group,
            buffer,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        let raw = InteractiveSectionRaw {
            0: Cuboid::new(self.transform.clone(), LinearSceneBvh::null_index_ptr()),
        };
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[raw]));
    }

    pub fn translate(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        result: &mut result::Result,
        size: winit::dpi::PhysicalSize<u32>,
        mouse_prev: Vector2<f32>,
        mouse_cur: Vector2<f32>,
    ) {
        // Update internal transformation matrix
        self.transform = self.transform
            * Matrix4::from_translation(Vector3::new(0.0, 0.0, mouse_cur.y - mouse_prev.y));

        // Push the changes to the GPU
        self.update(queue);
        // Reset the accumulation ray color result texture
        result.reset_texture(device, queue, size);
    }
}
