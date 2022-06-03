use cgmath::{prelude::*, Vector3};

use super::{bvh::Bvh, sphere::Sphere};

// pub struct Scene {
//     pub spheres: Vec<Sphere>,
// }

// impl Scene {
#[allow(dead_code)]
pub fn test_scene() -> Bvh {
    Bvh::build_from_spheres(&vec![
        Sphere::new(
            Vector3::<f32>::new(0.0, -100.5, -1.0),
            100.0,
            0,
            0.0,
            0.0,
            Vector3::<f32>::new(0.8, 0.8, 0.0),
        ),
        Sphere::new(
            Vector3::<f32>::new(0.0, 0.0, -1.0),
            0.5,
            0,
            0.0,
            0.0,
            Vector3::<f32>::new(0.1, 0.2, 0.5),
        ),
        Sphere::new(
            Vector3::<f32>::new(-1.0, 0.0, -1.0),
            0.5,
            2,
            0.0,
            1.5,
            Vector3::<f32>::new(0.0, 0.0, 0.0),
        ),
        Sphere::new(
            Vector3::<f32>::new(-1.0, 0.0, -1.0),
            -0.45,
            2,
            0.0,
            1.5,
            Vector3::<f32>::new(0.0, 0.0, 0.0),
        ),
        Sphere::new(
            Vector3::<f32>::new(1.0, 0.0, -1.0),
            0.5,
            1,
            0.0,
            0.0,
            Vector3::<f32>::new(0.8, 0.6, 0.2),
        ),
    ])
}

pub fn final_scene() -> Bvh {
    let mut spheres = Vec::<Sphere>::new();

    spheres.push(Sphere::new(
        Vector3::<f32>::new(0.0, -1000.0, -1.0),
        1000.0,
        0,
        0.0,
        0.0,
        Vector3::<f32>::new(0.5, 0.5, 0.5),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vector3::<f32>::new(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vector3::<f32>::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        0,
                        0.0,
                        0.0,
                        Vector3::<f32>::new(
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                        ),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        1,
                        rand::random::<f32>() * 0.5,
                        0.0,
                        Vector3::<f32>::new(
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                        ),
                    ));
                }
            } else {
                // glass
                spheres.push(Sphere::new(
                    center,
                    0.2,
                    2,
                    0.0,
                    1.5,
                    Vector3::<f32>::new(0.0, 0.0, 0.0),
                ));
            }
        }
    }

    spheres.push(Sphere::new(
        Vector3::<f32>::new(0.0, 1.0, 0.0),
        1.0,
        2,
        0.0,
        1.5,
        Vector3::<f32>::new(0.0, 0.0, 0.0),
    ));

    spheres.push(Sphere::new(
        Vector3::<f32>::new(-4.0, 1.0, 0.0),
        1.0,
        0,
        0.0,
        0.0,
        Vector3::<f32>::new(0.4, 0.2, 0.1),
    ));

    spheres.push(Sphere::new(
        Vector3::<f32>::new(4.0, 1.0, 0.0),
        1.0,
        1,
        0.0,
        0.0,
        Vector3::<f32>::new(0.7, 0.6, 0.5),
    ));

    Bvh::build_from_spheres(&spheres)
}
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_test_scene() {
        let scene = test_scene();
        assert!(scene.get_hittables().len() == 5);
    }

    #[test]
    fn test_final_scene() {
        let scene = final_scene();
        assert!(scene.get_hittables().len() > 5);
    }
}
