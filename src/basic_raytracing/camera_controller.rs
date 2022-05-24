use cgmath::Vector3;

use super::camera::Camera;

pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub struct CameraController {
    movement_speed: f32,
}

impl CameraController {
    pub fn new() -> Self {
        CameraController {
            movement_speed: 0.02,
        }
    }

    pub fn translate(&mut self, queue: &wgpu::Queue, camera: &mut Camera, direction: Direction) {
        match direction {
            Direction::Left => {
                camera.translate(queue, Vector3::new(-self.movement_speed, 0.0, 0.0));
            }
            Direction::Right => {
                camera.translate(queue, Vector3::new(self.movement_speed, 0.0, 0.0));
            }
            Direction::Forward => {
                camera.translate(queue, Vector3::new(0.0, 0.0, -self.movement_speed));
            }
            Direction::Backward => {
                camera.translate(queue, Vector3::new(0.0, 0.0, self.movement_speed));
            }
        }
    }
}
