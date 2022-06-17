use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};

use super::{aabb::Aabb, hittable::Hittable, linear_scene_bvh::LinearSceneBvh};

/// POD Rectangle ready to ship to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Cuboid {
    /// Centroid of the cuboid
    pub center: [f32; 3],
    pub _pad1: f32, // 8
    /// Axis aligned 'radius' (half edge length) of the cuboid
    pub radius: [f32; 3],
    /// Index of the material in the linear scene bvh
    pub material_index: u32, // 16
    /// World to object space transform
    pub txx: [[f32; 4]; 4],
    /// Object to world space transform
    pub txi: [[f32; 4]; 4],
}

impl Cuboid {
    pub fn new(
        center: Vector3<f32>,
        radius: Vector3<f32>,
        material_index: u32,
        txx: Matrix4<f32>,
        txi: Matrix4<f32>,
    ) -> Self {
        let txx_updated = txx
            // * Matrix4::from_nonuniform_scale(-radius.x, -radius.y, -radius.z)
            * Matrix4::from_translation(center * -1.0);

        let txi_computed = txx_updated.invert().unwrap();

        println!("txx_updated: {:?}", txx_updated);
        println!("txx_computed: {:?}", txi_computed);

        Cuboid {
            center: center.into(),
            _pad1: 0.0,
            radius: radius.into(),
            material_index,
            txx: txx_updated.into(),
            txi: txi_computed.into(),
        }
    }

    pub fn empty() -> Self {
        Cuboid {
            center: [0.0; 3],
            _pad1: 0.0,
            radius: [0.0; 3],
            material_index: LinearSceneBvh::null_index_ptr(),
            txx: [[0.0; 4]; 4],
            txi: [[0.0; 4]; 4],
        }
    }
}

impl Hittable for Cuboid {
    // TODO - fix this to account for the arbitrary transform matrix of the cuboid's orientation and scale
    fn bounding_box(&self) -> Aabb {
        Aabb::new(
            Point3::new(
                self.center[0] - 10.0 * self.radius[0],
                self.center[1] - 10.0 * self.radius[1],
                self.center[2] - 10.0 * self.radius[2],
            ),
            Point3::new(
                self.center[0] + 10.0 * self.radius[0],
                self.center[1] + 10.0 * self.radius[1],
                self.center[2] + 10.0 * self.radius[2],
            ),
        )
    }
}
