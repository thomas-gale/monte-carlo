use cgmath::{Matrix4, Vector2, Vector3};

use super::result;

pub struct InteractiveSection {
    transform: Matrix4<f32>,
    // buffer: wgpu::Buffer,
}

impl InteractiveSection {
    pub fn new(transform: Matrix4<f32>) -> Self {
        InteractiveSection {
            transform,
            // buffer,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
        let raw_trans: [[f32; 4]; 4] = self.transform.clone().into();
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[raw_trans]));
    }

    pub fn translate(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        result: &mut result::Result,
        size: winit::dpi::PhysicalSize<u32>,
        mouse_prev: Vector2<f32>,
        mouse_cur: Vector2<f32>,
    ) {
        // Update internal transformation matrix
        self.transform = self.transform
            * Matrix4::from_translation(Vector3::new(0.0, 0.0, mouse_cur.y - mouse_prev.y));

        println!("{:?}", self.transform);

        // Push the changes to the GPU
        self.update(queue, buffer);
        // Reset the accumulation ray color result texture
        result.reset_texture(device, queue, size);
    }
}
