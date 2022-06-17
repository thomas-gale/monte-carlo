use cgmath::{
    EuclideanSpace, Matrix3, Matrix4, Point3, Rad, SquareMatrix, Transform, Vector3, Vector4,
};

use super::{aabb::Aabb, hittable::Hittable, linear_scene_bvh::LinearSceneBvh};

/// POD Rectangle ready to ship to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Cuboid {
    /// Centroid of the cuboid
    // pub center: [f32; 3],
    // pub _pad1: f32, // 8
    /// Axis aligned 'radius' (half edge length) of the cuboid
    pub radius: [f32; 3],
    /// Index of the material in the linear scene bvh
    pub material_index: u32,
    /// World to object space transform (contains just the centroid)
    pub txx: [[f32; 4]; 4],
    /// Object to world space transform
    pub txi: [[f32; 4]; 4],
}

impl Cuboid {
    ///
    /// # Arguments
    /// * txi: Object to world space transform
    pub fn new(
        txi: Matrix4<f32>,
        // center: Vector3<f32>,
        radius: Vector3<f32>,
        material_index: u32,
        // txx: Matrix4<f32>,
    ) -> Self {
        // let txi_computed = txx_updated.invert().unwrap();

        // let txx_updated = txx
        //     // * Matrix4::from_angle_y(Rad(std::f32::consts::PI / 4.0))
        //     * Matrix4::from_translation(center * 1.0);

        // let txi_computed = txx_updated.invert().unwrap();

        // println!("txx_updated: {:?}", txx_updated);
        // println!("txi_computed: {:?}", txi_computed);

        Cuboid {
            // center: center.into(),
            // _pad1: 0.0,
            radius: radius.into(),
            material_index,
            // txx: txx_updated.into(),
            // txi: txi_computed.into(),
            txx: txi.invert().unwrap().into(),
            txi: txi.into(),
        }
    }

    pub fn empty() -> Self {
        Cuboid {
            // center: [0.0; 3],
            // _pad1: 0.0,
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
        // Naive
        // compute all 8 corners of the cuboid in world space

        let rotation = Matrix3::from_cols(
            Vector4::from(self.txi[0]).truncate(),
            Vector4::from(self.txi[1]).truncate(),
            Vector4::from(self.txi[2]).truncate(),
        );
        // .invert()
        // .unwrap();

        println!("rotation: {:?}", rotation);
        println!("rotation_inv: {:?}", rotation.invert().unwrap());

        // let rad_wrld = Matrix4::from(self.txx).transform_vector(Vector3::from(self.radius));
        // let rad_wrld = rotation.invert().unwrap() * Vector3::from(self.radius);
        // let rad_wrld = rotation * Vector3::from(self.radius);
        // let pos_wrld = rotation * Vector4::from(self.txi[3]).truncate();
        // let pos_wrld = rotation * Vector4::from(self.txi[3]).truncate();
        let pos_wrld = Vector4::from(self.txi[3]).truncate();

        // println!("rad_wrld: {:?}", rad_wrld);
        // println!("pos_wrld: {:?}", pos_wrld);

        // let c_0 = Vector3::new(
        //     pos_wrld.x - rad_wrld.x,
        //     pos_wrld.y - rad_wrld.y,
        //     pos_wrld.z - rad_wrld.z,
        // );
        // let c_1 = Vector3::new(
        //     pos_wrld.x - rad_wrld.x,
        //     pos_wrld.y - rad_wrld.y,
        //     pos_wrld.z + rad_wrld.z,
        // );

        // let c_2 = Vector3::new(
        //     pos_wrld.x - rad_wrld.x,
        //     pos_wrld.y + rad_wrld.y,
        //     pos_wrld.z - rad_wrld.z,
        // );

        // let c_3 = Vector3::new(
        //     pos_wrld.x - rad_wrld.x,
        //     pos_wrld.y + rad_wrld.y,
        //     pos_wrld.z + rad_wrld.z,
        // );

        // let c_4 = Vector3::new(
        //     pos_wrld.x + rad_wrld.x,
        //     pos_wrld.y - rad_wrld.y,
        //     pos_wrld.z - rad_wrld.z,
        // );

        // let c_5 = Vector3::new(
        //     pos_wrld.x + rad_wrld.x,
        //     pos_wrld.y - rad_wrld.y,
        //     pos_wrld.z + rad_wrld.z,
        // );

        // let c_6 = Vector3::new(
        //     pos_wrld.x + rad_wrld.x,
        //     pos_wrld.y + rad_wrld.y,
        //     pos_wrld.z - rad_wrld.z,
        // );

        // let c_7 = Vector3::new(
        //     pos_wrld.x + rad_wrld.x,
        //     pos_wrld.y + rad_wrld.y,
        //     pos_wrld.z + rad_wrld.z,
        // );

        // let c_0 = Vector3::new(
        //     self.txi[3][0] - rad_wrld.x,
        //     self.txi[3][1] - rad_wrld.y,
        //     self.txi[3][2] - rad_wrld.z,
        // );
        // let c_1 = Vector3::new(
        //     self.txi[3][0] - rad_wrld.x,
        //     self.txi[3][1] - rad_wrld.y,
        //     self.txi[3][2] + rad_wrld.z,
        // );

        // let c_2 = Vector3::new(
        //     self.txi[3][0] - rad_wrld.x,
        //     self.txi[3][1] + rad_wrld.y,
        //     self.txi[3][2] - rad_wrld.z,
        // );

        // let c_3 = Vector3::new(
        //     self.txi[3][0] - rad_wrld.x,
        //     self.txi[3][1] + rad_wrld.y,
        //     self.txi[3][2] + rad_wrld.z,
        // );

        // let c_4 = Vector3::new(
        //     self.txi[3][0] + rad_wrld.x,
        //     self.txi[3][1] - rad_wrld.y,
        //     self.txi[3][2] - rad_wrld.z,
        // );

        // let c_5 = Vector3::new(
        //     self.txi[3][0] + rad_wrld.x,
        //     self.txi[3][1] - rad_wrld.y,
        //     self.txi[3][2] + rad_wrld.z,
        // );

        // let c_6 = Vector3::new(
        //     self.txi[3][0] + rad_wrld.x,
        //     self.txi[3][1] + rad_wrld.y,
        //     self.txi[3][2] - rad_wrld.z,
        // );

        // let c_7 = Vector3::new(
        //     self.txi[3][0] + rad_wrld.x,
        //     self.txi[3][1] + rad_wrld.y,
        //     self.txi[3][2] + rad_wrld.z,
        // );

        // let c_0: Vector3<f32> = rotation
        //     * Vector3::new(
        //         self.txi[3][0] - self.radius[0],
        //         self.txi[3][1] - self.radius[1],
        //         self.txi[3][2] - self.radius[2],
        //     );
        // let c_1 = rotation
        //     * Vector3::new(
        //         self.txi[3][0] - self.radius[0],
        //         self.txi[3][1] - self.radius[1],
        //         self.txi[3][2] + self.radius[2],
        //     );
        // let c_2 = rotation
        //     * Vector3::new(
        //         self.txi[3][0] - self.radius[0],
        //         self.txi[3][1] + self.radius[1],
        //         self.txi[3][2] - self.radius[2],
        //     );
        // let c_3 = rotation
        //     * Vector3::new(
        //         self.txi[3][0] - self.radius[0],
        //         self.txi[3][1] + self.radius[1],
        //         self.txi[3][2] + self.radius[2],
        //     );
        // let c_4 = rotation
        //     * Vector3::new(
        //         self.txi[3][0] + self.radius[0],
        //         self.txi[3][1] - self.radius[1],
        //         self.txi[3][2] - self.radius[2],
        //     );
        // let c_5 = rotation
        //     * Vector3::new(
        //         self.txi[3][0] + self.radius[0],
        //         self.txi[3][1] - self.radius[1],
        //         self.txi[3][2] + self.radius[2],
        //     );
        // let c_6 = rotation
        //     * Vector3::new(
        //         self.txi[3][0] + self.radius[0],
        //         self.txi[3][1] + self.radius[1],
        //         self.txi[3][2] - self.radius[2],
        //     );
        // let c_7 = rotation
        //     * Vector3::new(
        //         self.txi[3][0] + self.radius[0],
        //         self.txi[3][1] + self.radius[1],
        //         self.txi[3][2] + self.radius[2],
        //     );

        let c_0: Vector3<f32> =
            rotation * Vector3::new(-self.radius[0], -self.radius[1], -self.radius[2]);
        let c_1 = rotation * Vector3::new(-self.radius[0], -self.radius[1], self.radius[2]);
        let c_2 = rotation * Vector3::new(-self.radius[0], self.radius[1], -self.radius[2]);
        let c_3 = rotation * Vector3::new(-self.radius[0], self.radius[1], self.radius[2]);
        let c_4 = rotation * Vector3::new(self.radius[0], -self.radius[1], -self.radius[2]);
        let c_5 = rotation * Vector3::new(self.radius[0], -self.radius[1], self.radius[2]);
        let c_6 = rotation * Vector3::new(self.radius[0], self.radius[1], -self.radius[2]);
        let c_7 = rotation * Vector3::new(self.radius[0], self.radius[1], self.radius[2]);

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

        Aabb::new(
            // min,
            Point3::from_vec(pos_wrld + min.to_vec()),
            // min - Vector4::from(self.txx[3]).truncate(),
            // min + Vector4::from(self.txx[3]).truncate(),
            // max
            Point3::from_vec(pos_wrld + max.to_vec()),
            // max + Vector4::from(self.txx[3]).truncate(),
            // max + Vector4::from(self.txx[3]).truncate(),
        )

        // compute min and max in object space
        // let min = Vector3::new(
        //     self.txx[3][0] - self.radius[0],
        //     self.txx[3][1] - self.radius[1],
        //     self.txx[3][2] - self.radius[2],
        // );
        // let max = Vector3::new(
        //     self.txx[3][0] + self.radius[0],
        //     self.txx[3][1] + self.radius[1],
        //     self.txx[3][2] + self.radius[2],
        // );

        // transform radius to world space
        // let rad_wrld = Matrix4::from(self.txi).transform_vector(Vector3::from(self.radius));

        // return aabb (with min/max transformed to world space)
        // let aabb = Aabb::new(
        //     // Point3::from_vec((Matrix4::from(self.txi) * min.extend(1.0)).truncate()),
        //     // Point3::from_vec((Matrix4::from(self.txi) * max.extend(1.0)).truncate()),
        //     Point3::new(
        //         self.txi[3][0] - rad_wrld.x,
        //         self.txi[3][1] - rad_wrld.y,
        //         self.txi[3][2] - rad_wrld.z,
        //     ),
        //     Point3::new(
        //         self.txi[3][0] + rad_wrld.x,
        //         self.txi[3][1] + rad_wrld.y,
        //         self.txi[3][2] + rad_wrld.z,
        //         // self.center[0] + 10.0 * self.radius[0],
        //         // self.center[1] + 10.0 * self.radius[1],
        //         // self.center[2] + 10.0 * self.radius[2],
        //     ),
        // );

        // println!("Cuboid bounding box: {:?}", aabb);

        // aabb
    }
}
