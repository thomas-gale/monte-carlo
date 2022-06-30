use cgmath::{Matrix4, Vector2, Vector3};

use super::{linear_hittable::LinearHittable, linear_scene_bvh::LinearSceneBvh, result};

pub struct InteractiveSection {
    hittable: LinearHittable,
}

impl InteractiveSection {
    pub fn new(hittable: LinearHittable) -> Self {
        InteractiveSection { hittable }
    }

    pub fn translate(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &mut LinearSceneBvh,
        result: &mut result::Result,
        size: winit::dpi::PhysicalSize<u32>,
        mouse_prev: Vector2<f32>,
        mouse_cur: Vector2<f32>,
    ) {
        // Update the scene
        scene.transform_hittable_by(
            queue,
            &self.hittable,
            Matrix4::from_translation(Vector3::new(0.0, 0.0, (mouse_cur.y - mouse_prev.y) * 0.1)),
        );

        // Reset the accumulation ray color result texture
        result.reset_texture(device, queue, size);
    }
}
