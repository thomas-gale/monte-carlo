use cgmath::{Point3, Vector3};

use super::{aabb::Aabb, linear_scene_bvh::LinearSceneBvh};

///
/// POD Sphere ready to ship to GPU
///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    /// Index of the material in the linear scene bvh
    pub material_index: u32,
    pub _pad1: f32,
    pub _pad2: f32,
    pub _pad3: f32,
}

impl Sphere {
    pub fn new(
        center: Vector3<f32>,
        radius: f32,
        material_index: u32,
    ) -> Self {
        Sphere {
            center: center.into(),
            radius,
            material_index,
            _pad1: 0.0,
            _pad2: 0.0,
            _pad3: 0.0,
        }
    }

    pub fn empty() -> Self {
        Sphere {
            center: [0.0; 3],
            radius: 0.0,
            material_index: LinearSceneBvh::null_index_ptr(),
            _pad1: 0.0,
            _pad2: 0.0,
            _pad3: 0.0,
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        Aabb::new(
            Point3::new(
                self.center[0] - self.radius,
                self.center[1] - self.radius,
                self.center[2] - self.radius,
            ),
            Point3::new(
                self.center[0] + self.radius,
                self.center[1] + self.radius,
                self.center[2] + self.radius,
            ),
        )
    }
}
