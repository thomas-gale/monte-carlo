use super::sphere::Sphere;

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![
                Sphere {
                    center: [0.0, -100.25, -1.0],
                    radius: 100.0,
                    albedo: [0.8, 0.8, 0.2],
                    material_type: 0,
                },
                Sphere {
                    center: [0.0, 0.0, -1.0],
                    radius: 0.25,
                    albedo: [0.7, 0.3, 0.3],
                    material_type: 0,
                },
                Sphere {
                    center: [-0.5, 0.0, -1.0],
                    radius: 0.25,
                    albedo: [0.8, 0.8, 0.8],
                    material_type: 1,
                },
                Sphere {
                    center: [0.5, 0.0, -1.0],
                    radius: 0.25,
                    albedo: [0.8, 0.6, 0.2],
                    material_type: 1,
                },
            ],
        }
    }
}
