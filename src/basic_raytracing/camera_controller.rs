use wgpu::util::DeviceExt;

use super::{buffer_bindings, camera::Camera};

pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub struct CameraController {
    camera: Camera,
    movement_speed: f32,
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
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Uniform,
        );

        CameraController {
            camera,
            movement_speed: 0.02,
            bind_group_layout,
            bind_group,
            buffer,
        }
    }

    pub fn translate(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, direction: Direction) {
        match direction {
            Direction::Left => {
                self.camera.origin[0] -= self.movement_speed;
                self.camera.lower_left_corner[0] -= self.movement_speed;
            }
            Direction::Right => {
                self.camera.origin[0] += self.movement_speed;
                self.camera.lower_left_corner[0] += self.movement_speed;
            }
            Direction::Forward => {
                self.camera.origin[2] -= self.movement_speed;
                self.camera.lower_left_corner[2] -= self.movement_speed;
            }
            Direction::Backward => {
                self.camera.origin[2] += self.movement_speed;
                self.camera.lower_left_corner[2] += self.movement_speed;
            }
        }
        let new_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[self.camera]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_SRC,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("update camera buffer"),
        });

        encoder.copy_buffer_to_buffer(
            &new_camera_buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<Camera>() as wgpu::BufferAddress,
        );

        queue.submit(std::iter::once(encoder.finish()));
    }
}
