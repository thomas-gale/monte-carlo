use super::sphere::Sphere;

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![
                Sphere {
                    center: [0.0, 0.0, -1.0],
                    radius: 0.25,
                },
                Sphere {
                    center: [0.0, -100.25, -1.0],
                    radius: 100.0,
                },
            ],
        }
    }
}
