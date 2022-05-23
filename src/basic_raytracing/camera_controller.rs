use super::{buffer_bindings, camera::Camera};

pub struct CameraController {
    camera: Camera,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl CameraController {
    pub fn new<'a>(device: &wgpu::Device) -> Self {
        let camera = Camera::new();
        let (bind_group_layout, bind_group, buffer) = buffer_bindings::create_device_buffer_binding(
            &[camera],
            &device,
            wgpu::BufferUsages::UNIFORM,
            wgpu::BufferBindingType::Uniform,
        );

        CameraController {
            camera,
            bind_group_layout,
            bind_group,
            buffer,
        }
    }

    pub fn delta_x_tranlate_origin(&mut self, delta_distance: f32) {
        self.camera.origin[0] += delta_distance;
    }

    // fn update_buffer(&mut self, device: &wgpu::Device) {
    //     buffer_bindin
    // }
}
