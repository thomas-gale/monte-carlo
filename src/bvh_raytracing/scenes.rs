use cgmath::{prelude::*, Vector3};

use super::{
    construction_scene, linear_scene_bvh::LinearSceneBvh, material::Material, sphere::Sphere,
};

#[allow(dead_code)]
pub fn simple_scene() -> LinearSceneBvh {
    construction_scene::build_from_spheres(
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
    construction_scene::build_from_spheres(
        &vec![
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.8, 0.8, 0.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.1, 0.2, 0.5)),
            Material::new(1, 0.0, 1.5, Vector3::<f32>::new(1.0, 0.0, 0.0)),
            Material::new(1, 0.0, 1.5, Vector3::<f32>::new(0.0, 0.0, 1.0)),
            Material::new(2, 0.0, 1.5, Vector3::<f32>::new(0.0, 0.0, 0.0)),
        ],
        &vec![
            Sphere::new(Vector3::<f32>::new(0.0, -100.5, 0.0), 100.0, 0),
            Sphere::new(Vector3::<f32>::new(0.0, 0.0, 0.0), 0.5, 1),
            Sphere::new(Vector3::<f32>::new(0.0, 0.0, -1.0), 0.5, 3),
            Sphere::new(Vector3::<f32>::new(0.0, 0.0, 1.0), 0.5, 3),
            Sphere::new(Vector3::<f32>::new(-1.0, 0.0, 0.0), 0.5, 4),
            Sphere::new(Vector3::<f32>::new(-1.0, 0.0, 0.0), -0.45, 4),
            Sphere::new(Vector3::<f32>::new(1.0, 0.0, 0.0), 0.5, 2),
        ],
    )
}

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
