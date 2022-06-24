use std::cmp::{max, min};

use cgmath::{
    EuclideanSpace, InnerSpace, Matrix3, Matrix4, Point3, SquareMatrix, Vector3, Vector4,
};

use super::{aabb::Aabb, linear_scene_bvh::LinearSceneBvh};

/// POD Rectangle ready to ship to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Cuboid {
    /// Index of the material in the linear scene bvh
    pub material_index: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
    /// World to object space transform (computed automatically as inverse of txi)
    pub txx: [[f32; 4]; 4],
    /// Object to world space transform (place translations/scales/rotations into this matrix)
    pub txi: [[f32; 4]; 4],
}

impl Cuboid {
    ///
    /// Construct a new cuboid
    /// * `txi` - Object to world space transform (place translations/scales/rotations into this matrix)
    /// * `material_index` - Index of the material in the linear scene bvh
    pub fn new(txi: Matrix4<f32>, material_index: u32) -> Self {
        Cuboid {
            material_index,
            _pad1: 0,
            _pad2: 0,
            _pad3: 0,
            txx: txi.invert().unwrap().into(),
            txi: txi.into(),
        }
    }

    pub fn empty() -> Self {
        Cuboid {
            material_index: LinearSceneBvh::null_index_ptr(),
            _pad1: 0,
            _pad2: 0,
            _pad3: 0,
            txx: [[0.0; 4]; 4],
            txi: [[0.0; 4]; 4],
        }
    }

    /// Returns the bounding box of the cuboid.
    /// Assumes transformation matrix basis vectors are orthogonal to one another (no skews)
    pub fn bounding_box(&self) -> Aabb {
        // the cuboids transformation matrix from local to world space
        let mut rotation = Matrix3::from_cols(
            Vector4::from(self.txi[0]).truncate(),
            Vector4::from(self.txi[1]).truncate(),
            Vector4::from(self.txi[2]).truncate(),
        );
        let scale = Vector3::new(
            rotation.x.magnitude(),
            rotation.y.magnitude(),
            rotation.z.magnitude(),
        );
        rotation = Matrix3::from_cols(
            rotation.x.normalize(),
            rotation.y.normalize(),
            rotation.z.normalize(),
        );

        // Compute all 8 corners of the cuboid in world space.
        let corners: Vec<Vector3<f32>> = (0..8)
            .map(|i| {
                let x_sgn: f32 = if i & (1 << 2) > 0 { 1.0 } else { -1.0 };
                let y_sgn: f32 = if i & (1 << 1) > 0 { 1.0 } else { -1.0 };
                let z_sgn: f32 = if i & (1 << 0) > 0 { 1.0 } else { -1.0 };
                rotation * Vector3::new(x_sgn * scale.x, y_sgn * scale.y, z_sgn * scale.z)
            })
            .collect();

        // Find the min and max corner
        let (min, max) = corners.iter().fold(
            (
                Point3::new(f32::MAX, f32::MAX, f32::MAX),
                Point3::new(f32::MIN, f32::MIN, f32::MIN),
            ),
            |(cur_min, cur_max), corner| {
                (
                    Point3::new(
                        f32::min(cur_min.x, corner.x),
                        f32::min(cur_min.y, corner.y),
                        f32::min(cur_min.z, corner.z),
                    ),
                    Point3::new(
                        f32::max(cur_max.x, corner.x),
                        f32::max(cur_max.y, corner.y),
                        f32::max(cur_max.z, corner.z),
                    ),
                )
            },
        );

        // vector version of centroid of the cuboid in world space.
        let pos_wrld = Vector4::from(self.txi[3]).truncate();

        Aabb::new(
            Point3::from_vec(pos_wrld + min.to_vec()),
            Point3::from_vec(pos_wrld + max.to_vec()),
        )
    }
}
