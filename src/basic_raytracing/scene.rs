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
                    material_type: 0,
                    fuzz: 0.0,
                    refraction_index: 0.0,
                    _pad1: 0.0,
                    albedo: [0.8, 0.8, 0.2],
                    _pad2: 0.0,
                },
                Sphere {
                    center: [0.0, 0.0, -1.0],
                    radius: 0.25,
                    material_type: 0,
                    fuzz: 0.0,
                    refraction_index: 0.0,
                    _pad1: 0.0,
                    albedo: [0.7, 0.3, 0.3],
                    _pad2: 0.0,
                },
                Sphere {
                    center: [-0.5, 0.0, -1.0],
                    radius: 0.25,
                    material_type: 2,
                    fuzz: 0.3,
                    refraction_index: 1.5,
                    _pad1: 0.0,
                    albedo: [0.8, 0.8, 0.8],
                    _pad2: 0.0,
                },
                Sphere {
                    center: [0.5, 0.0, -1.0],
                    radius: 0.25,
                    material_type: 1,
                    fuzz: 1.0,
                    refraction_index: 0.0,
                    _pad1: 0.0,
                    albedo: [0.8, 0.6, 0.2],
                    _pad2: 0.0,
                },
            ],
        }
    }
}
