use cgmath::{
    EuclideanSpace, Matrix3, Matrix4, Point3, Rad, SquareMatrix, Transform, Vector3, Vector4,
};

use super::{aabb::Aabb, hittable::Hittable, linear_scene_bvh::LinearSceneBvh};

/// POD Rectangle ready to ship to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Cuboid {
    /// Axis aligned 'radius' (half edge length) of the cuboid
    pub radius: [f32; 3],
    /// Index of the material in the linear scene bvh
    pub material_index: u32,
    /// World to object space transform (computed automatically as inverse of txi)
    pub txx: [[f32; 4]; 4],
    /// Object to world space transform
    pub txi: [[f32; 4]; 4],
}

impl Cuboid {
    ///
    /// Construct a new cuboid
    /// * `txi` - Object to world space transform ()
    /// * `radius` - Axis aligned 'radius' (half edge length) of the cuboid
    /// * `material_index` - Index of the material in the linear scene bvh
    pub fn new(txi: Matrix4<f32>, radius: Vector3<f32>, material_index: u32) -> Self {
        Cuboid {
            radius: radius.into(),
            material_index,
            txx: txi.invert().unwrap().into(),
            txi: txi.into(),
        }
    }

    pub fn empty() -> Self {
        Cuboid {
            radius: [0.0; 3],
            material_index: LinearSceneBvh::null_index_ptr(),
            txx: [[0.0; 4]; 4],
            txi: [[0.0; 4]; 4],
        }
    }
}

impl Hittable for Cuboid {
    /// Returns the bounding box of the cuboid.
    /// TODO - some tidying to reduce line count can be done as highlighted in comments within function
    fn bounding_box(&self) -> Aabb {
        // the cuboids transformation matrix from local to world space
        let rotation = Matrix3::from_cols(
            Vector4::from(self.txi[0]).truncate(),
            Vector4::from(self.txi[1]).truncate(),
            Vector4::from(self.txi[2]).truncate(),
        );

        // TODO - remove duplicate code and use a simple 3 bit mask on a loop (much more elegant)

        // Naive, compute all 8 corners of the cuboid in world space.
        let c_0: Vector3<f32> =
            rotation * Vector3::new(-self.radius[0], -self.radius[1], -self.radius[2]);
        let c_1 = rotation * Vector3::new(-self.radius[0], -self.radius[1], self.radius[2]);
        let c_2 = rotation * Vector3::new(-self.radius[0], self.radius[1], -self.radius[2]);
        let c_3 = rotation * Vector3::new(-self.radius[0], self.radius[1], self.radius[2]);
        let c_4 = rotation * Vector3::new(self.radius[0], -self.radius[1], -self.radius[2]);
        let c_5 = rotation * Vector3::new(self.radius[0], -self.radius[1], self.radius[2]);
        let c_6 = rotation * Vector3::new(self.radius[0], self.radius[1], -self.radius[2]);
        let c_7 = rotation * Vector3::new(self.radius[0], self.radius[1], self.radius[2]);

        // The min and max possible corner coordinates.
        let min = Point3::new(
            c_0.x
                .min(c_1.x)
                .min(c_2.x)
                .min(c_3.x)
                .min(c_4.x)
                .min(c_5.x)
                .min(c_6.x)
                .min(c_7.x),
            c_0.y
                .min(c_1.y)
                .min(c_2.y)
                .min(c_3.y)
                .min(c_4.y)
                .min(c_5.y)
                .min(c_6.y)
                .min(c_7.y),
            c_0.z
                .min(c_1.z)
                .min(c_2.z)
                .min(c_3.z)
                .min(c_4.z)
                .min(c_5.z)
                .min(c_6.z)
                .min(c_7.z),
        );

        let max = Point3::new(
            c_0.x
                .max(c_1.x)
                .max(c_2.x)
                .max(c_3.x)
                .max(c_4.x)
                .max(c_5.x)
                .max(c_6.x)
                .max(c_7.x),
            c_0.y
                .max(c_1.y)
                .max(c_2.y)
                .max(c_3.y)
                .max(c_4.y)
                .max(c_5.y)
                .max(c_6.y)
                .max(c_7.y),
            c_0.z
                .max(c_1.z)
                .max(c_2.z)
                .max(c_3.z)
                .max(c_4.z)
                .max(c_5.z)
                .max(c_6.z)
                .max(c_7.z),
        );

        // vector version of centroid of the cuboid in world space.
        let pos_wrld = Vector4::from(self.txi[3]).truncate();

        Aabb::new(
            Point3::from_vec(pos_wrld + min.to_vec()),
            Point3::from_vec(pos_wrld + max.to_vec()),
        )
    }
}
