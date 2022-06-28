use cgmath::{prelude::*, Deg, Matrix4, Vector3};

use super::{
    constant_medium::ConstantMedium, construction_scene, cuboid::Cuboid,
    hittable_primitive::HittablePrimitive, linear_scene_bvh::LinearSceneBvh, material::Material,
    sphere::Sphere,
};

#[allow(dead_code)]
pub fn simple_scene() -> LinearSceneBvh {
    construction_scene::build_from_spheres(
        Material::new(0, 0.0, 0.0, Vector3::new(0.70, 0.80, 1.00)),
        &vec![Material::new(
            0,
            0.0,
            0.0,
            Vector3::<f32>::new(1.0, 0.0, 0.0),
        )],
        &vec![Sphere::new(Vector3::<f32>::new(0.0, 0.0, 0.0), 0.5, 0)],
    )
}

#[allow(dead_code)]
pub fn test_scene() -> LinearSceneBvh {
    construction_scene::build_from_hittable_primitives(
        Material::new(0, 0.0, 0.0, Vector3::new(0.70, 0.80, 1.00)),
        &vec![
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.4, 0.4, 0.8)), // 0
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.1, 0.2, 0.5)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(1.0, 0.0, 0.0)), // 2
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.0, 1.0, 0.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.0, 0.0, 1.0)), // 4
            Material::new(2, 0.0, 1.5, Vector3::<f32>::new(0.0, 0.0, 0.0)),
            Material::new(4, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 0.2)), // 6
            Material::new(3, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 8.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.8, 0.3, 0.5)), // 8
            Material::new(5, 0.0, 0.0, Vector3::<f32>::new(0.0, 0.0, 0.0)),
        ],
        &vec![
            // HittablePrimitive::Cuboid(Cuboid::new(
            //     Matrix4::identity()
            //         * Matrix4::from_translation(Vector3::new(0.0, 1.0, 0.0))
            //         * Matrix4::from_nonuniform_scale(5.0, 5.0, 0.1),
            //     9,
            // )),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(0.0, 0.0, -5.0), 5.0, 9)),
            // HittablePrimitive::Cuboid(Cuboid::new(
            //     Matrix4::identity()
            //         * Matrix4::from_translation(Vector3::new(0.0, -1.0, 0.0))
            //         * Matrix4::from_nonuniform_scale(100.0, 0.5, 100.0),
            //     0,
            // )),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(2.0, 0.0, 0.0), 0.75, 2)),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(-2.0, 0.0, 0.0), 1.25, 3)),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(0.0, 0.0, 2.0), 0.5, 4)),
            // HittablePrimitive::ConstantMedium(ConstantMedium {
            //     boundary_hittable: Box::new(HittablePrimitive::Sphere(Sphere::new(
            //         Vector3::<f32>::new(0.0, 0.0, -2.5),
            //         0.5,
            //         LinearSceneBvh::null_index_ptr(),
            //     ))),
            //     material_index: 6,
            //     density: 2.0,
            // }),
            // HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(0.0, 0.0, -2.5), 0.1, 7)),
            // HittablePrimitive::Cuboid(Cuboid::new(
            //     Matrix4::identity()
            //         * Matrix4::from_angle_y(Deg(20.0))
            //         * Matrix4::from_nonuniform_scale(0.5, 0.5, 0.5),
            //     8,
            // )),
            // HittablePrimitive::Cuboid(Cuboid::new(
            //     Matrix4::identity()
            //         * Matrix4::from_translation(Vector3::new(-1.5, 1.0, -2.0))
            //         * Matrix4::from_angle_x(Deg(10.0))
            //         * Matrix4::from_angle_y(Deg(20.0))
            //         * Matrix4::from_angle_z(Deg(40.0))
            //         * Matrix4::from_nonuniform_scale(0.5, 0.75, 1.0),
            //     1,
            // )),
        ],
    )
}

// #[allow(dead_code)]
// pub fn cornell_box() -> LinearSceneBvh {
//     construction_scene::build_from_hittable_primitives(
//         Material::new(0, 0.0, 0.0, Vector3::new(0.0, 0.0, 0.0)),
//         &vec![
//             Material::new(0, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 1.0)),
//             Material::new(0, 0.0, 0.0, Vector3::<f32>::new(1.0, 0.0, 0.0)),
//             Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.0, 1.0, 0.0)),
//             Material::new(3, 0.0, 0.0, Vector3::<f32>::new(2.0, 2.0, 2.0)),
//             Material::new(2, 0.0, 1.5, Vector3::<f32>::new(0.0, 0.0, 0.0)),
//             Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.9, 0.9, 0.9)),
//             Material::new(4, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 0.0)),
//         ],
//         &vec![
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity() * Matrix4::from_translation(Vector3::new(0.0, -0.01, 0.0)),
//                 Vector3::new(0.5, 0.01, 0.5),
//                 0,
//             )),
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity() * Matrix4::from_translation(Vector3::new(0.0, 1.01, 0.0)),
//                 Vector3::new(0.5, 0.01, 0.5),
//                 0,
//             )),
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity() * Matrix4::from_translation(Vector3::new(0.0, 0.5, -0.51)),
//                 Vector3::new(0.5, 0.5, 0.01),
//                 0,
//             )),
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity() * Matrix4::from_translation(Vector3::new(-0.51, 0.5, 0.0)),
//                 Vector3::new(0.01, 0.5, 0.5),
//                 1,
//             )),
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity() * Matrix4::from_translation(Vector3::new(0.51, 0.5, 0.0)),
//                 Vector3::new(0.01, 0.5, 0.5),
//                 2,
//             )),
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity() * Matrix4::from_translation(Vector3::new(0.0, 0.9999, 0.0)),
//                 Vector3::new(0.25, 0.0001, 0.25),
//                 3,
//             )),
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity()
//                     * Matrix4::from_translation(Vector3::new(-0.25, 0.3, -0.25))
//                     * Matrix4::from_angle_y(Deg(20.0)),
//                 Vector3::new(0.125, 0.3, 0.125),
//                 5,
//             )),
//             HittablePrimitive::Cuboid(Cuboid::new(
//                 Matrix4::identity()
//                     * Matrix4::from_translation(Vector3::new(0.125, 0.125, 0.25))
//                     * Matrix4::from_angle_y(Deg(-20.0)),
//                 Vector3::new(0.125, 0.125, 0.125),
//                 5,
//             )),
//             // HittablePrimitive::ConstantMedium(ConstantMedium {
//             //     boundary_hittable: Box::new(HittablePrimitive::Cuboid(Cuboid::new(
//             //         Matrix4::identity()
//             //             * Matrix4::from_translation(Vector3::new(0.125, 0.125, 0.25))
//             //             * Matrix4::from_angle_y(Deg(-20.0)),
//             //         Vector3::new(0.125, 0.125, 0.125),
//             //         LinearSceneBvh::null_index_ptr(),
//             //     ))),
//             //     material_index: 6,
//             //     density: 5.0,
//             // }),
//             HittablePrimitive::Sphere(Sphere::new(
//                 Vector3::<f32>::new(-0.125, 0.125, 0.125),
//                 0.125,
//                 4,
//             )),
//             // HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(0.24, 0.25, -0.24), 0.25, 5)),
//             HittablePrimitive::ConstantMedium(ConstantMedium {
//                 boundary_hittable: Box::new(HittablePrimitive::Sphere(Sphere::new(
//                     Vector3::<f32>::new(0.24, 0.25, -0.24),
//                     0.25,
//                     5,
//                 ))),
//                 material_index: 6,
//                 density: 5.0,
//             }),
//             // HittablePrimitive::Sphere(Sphere::new(
//             //     Vector3::<f32>::new(0.0, 0.125, 0.125),
//             //     0.125,
//             //     4,
//             // )),
//             // HittablePrimitive::Sphere(Sphere::new(
//             //     Vector3::<f32>::new(-0.25, 0.125, -0.125),
//             //     0.125,
//             //     4,
//             // )),
//             // HittablePrimitive::Sphere(Sphere::new(
//             //     Vector3::<f32>::new(0.25, 0.125, -0.125),
//             //     0.125,
//             //     4,
//             // )),
//         ],
//     )
// }

// #[allow(dead_code)]
// pub fn final_scene() -> LinearSceneBvh {
//     let mut spheres = Vec::<Sphere>::new();

//     spheres.push(Sphere::new(
//         Vector3::<f32>::new(0.0, -1000.0, -1.0),
//         1000.0,
//         0,
//         0.0,
//         0.0,
//         Vector3::<f32>::new(0.5, 0.5, 0.5),
//     ));

//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = rand::random::<f32>();
//             let center = Vector3::<f32>::new(
//                 a as f32 + 0.9 * rand::random::<f32>(),
//                 0.2,
//                 b as f32 + 0.9 * rand::random::<f32>(),
//             );

//             if (center - Vector3::<f32>::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
//                 if choose_mat < 0.6 {
//                     // diffuse
//                     spheres.push(Sphere::new(
//                         center,
//                         0.2,
//                         0,
//                         0.0,
//                         0.0,
//                         Vector3::<f32>::new(
//                             rand::random::<f32>() * rand::random::<f32>(),
//                             rand::random::<f32>() * rand::random::<f32>(),
//                             rand::random::<f32>() * rand::random::<f32>(),
//                         ),
//                     ));
//                 } else if choose_mat < 0.9 {
//                     // metal
//                     spheres.push(Sphere::new(
//                         center,
//                         0.2,
//                         1,
//                         rand::random::<f32>() * 0.5,
//                         0.0,
//                         Vector3::<f32>::new(
//                             0.5 * (1.0 + rand::random::<f32>()),
//                             0.5 * (1.0 + rand::random::<f32>()),
//                             0.5 * (1.0 + rand::random::<f32>()),
//                         ),
//                     ));
//                 }
//             } else {
//                 // glass
//                 spheres.push(Sphere::new(
//                     center,
//                     0.2,
//                     2,
//                     0.0,
//                     1.5,
//                     Vector3::<f32>::new(0.0, 0.0, 0.0),
//                 ));
//             }
//         }
//     }

//     spheres.push(Sphere::new(
//         Vector3::<f32>::new(0.0, 1.0, 0.0),
//         1.0,
//         2,
//         0.0,
//         1.5,
//         Vector3::<f32>::new(0.0, 0.0, 0.0),
//     ));

//     spheres.push(Sphere::new(
//         Vector3::<f32>::new(-4.0, 1.0, 0.0),
//         1.0,
//         0,
//         0.0,
//         0.0,
//         Vector3::<f32>::new(0.4, 0.2, 0.1),
//     ));

//     spheres.push(Sphere::new(
//         Vector3::<f32>::new(4.0, 1.0, 0.0),
//         1.0,
//         1,
//         0.0,
//         0.0,
//         Vector3::<f32>::new(0.7, 0.6, 0.5),
//     ));

//     construction_scene::build_from_spheres(&spheres)
// }

// #[allow(dead_code)]
// pub fn stress_test_scene() -> LinearSceneBvh {
//     let mut spheres = Vec::<Sphere>::new();

//     for a in -32..32 {
//         for b in -32..32 {
//             for c in -32..32 {
//                 let choose_mat = rand::random::<f32>();
//                 let center = Vector3::<f32>::new(
//                     2.0 * a as f32 + 1.9 * rand::random::<f32>(),
//                     2.0 * c as f32 + 1.9 * rand::random::<f32>(),
//                     2.0 * b as f32 + 1.9 * rand::random::<f32>(),
//                 );

//                 if (center - Vector3::<f32>::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
//                     if choose_mat < 0.1 {
//                         // diffuse
//                         spheres.push(Sphere::new(
//                             center,
//                             0.2,
//                             0,
//                             0.0,
//                             0.0,
//                             Vector3::<f32>::new(
//                                 rand::random::<f32>() * rand::random::<f32>(),
//                                 rand::random::<f32>() * rand::random::<f32>(),
//                                 rand::random::<f32>() * rand::random::<f32>(),
//                             ),
//                         ));
//                     } else if choose_mat < 0.8 {
//                         // metal
//                         spheres.push(Sphere::new(
//                             center,
//                             0.2,
//                             1,
//                             rand::random::<f32>() * 0.5,
//                             0.0,
//                             Vector3::<f32>::new(
//                                 0.5 * (1.0 + rand::random::<f32>()),
//                                 0.5 * (1.0 + rand::random::<f32>()),
//                                 0.5 * (1.0 + rand::random::<f32>()),
//                             ),
//                         ));
//                     }
//                 } else {
//                     // glass
//                     spheres.push(Sphere::new(
//                         center,
//                         0.2,
//                         2,
//                         0.0,
//                         1.5,
//                         Vector3::<f32>::new(0.0, 0.0, 0.0),
//                     ));
//                 }
//             }
//         }
//     }

//     spheres.push(Sphere::new(
//         Vector3::<f32>::new(0.0, 1.0, 0.0),
//         1.0,
//         2,
//         0.0,
//         1.5,
//         Vector3::<f32>::new(0.0, 0.0, 0.0),
//     ));

//     spheres.push(Sphere::new(
//         Vector3::<f32>::new(-4.0, 1.0, 0.0),
//         1.0,
//         0,
//         0.0,
//         0.0,
//         Vector3::<f32>::new(0.4, 0.2, 0.1),
//     ));

//     spheres.push(Sphere::new(
//         Vector3::<f32>::new(4.0, 1.0, 0.0),
//         1.0,
//         1,
//         0.0,
//         0.0,
//         Vector3::<f32>::new(0.7, 0.6, 0.5),
//     ));

//     construction_scene::build_from_spheres(&spheres)
// }
