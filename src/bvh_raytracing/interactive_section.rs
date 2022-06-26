use cgmath::{Matrix4, Vector2, Vector3};

use crate::bvh_raytracing::construction_scene::recompute_bvh;

use super::{linear_hittable::LinearHittable, linear_scene_bvh::LinearSceneBvh, result};

pub struct InteractiveSection {
    hittable: LinearHittable,
    /// Transformation applied interactively to this section
    transform: Matrix4<f32>,
}

impl InteractiveSection {
    pub fn new(hittable: LinearHittable, transform: Matrix4<f32>) -> Self {
        InteractiveSection {
            hittable,
            transform,
        }
    }

    // pub fn update(&mut self, queue: &wgpu::Queue, scene: &LinearSceneBvh) {
    //     let raw_trans: [[f32; 4]; 4] = self.transform.clone().into();
    //     queue.write_buffer(
    //         scene.interactive_transform_buffer.as_ref().unwrap(),
    //         0,
    //         bytemuck::cast_slice(&[raw_trans]),
    //     );
    // }

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
        // Update internal transformation matrix
        self.transform = self.transform
            * Matrix4::from_translation(Vector3::new(0.0, 0.0, mouse_cur.y - mouse_prev.y));

        println!("{:?}", self.transform);

        // Update the scene
        scene.transform_hittable_by(queue, &self.hittable, self.transform);

        // Reset the accumulation ray color result texture
        result.reset_texture(device, queue, size);
    }
}
