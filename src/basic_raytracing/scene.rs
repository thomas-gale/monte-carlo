use cgmath::Vector3;

use super::sphere::Sphere;

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![
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
            ],
        }
    }
}
