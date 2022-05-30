use super::{hittable::*, sphere::Sphere};

///
/// The basic linearized version of the scene, ready to be transferred to the GPU
///
pub struct Bvh {
    hittables: Vec<Hittable>,
}

impl Bvh {
    /// Experimental function to build a BVH from a slice of spheres
    pub fn build_from_spheres(spheres: &[Sphere]) -> Self {
        Bvh {
            hittables: spheres
                .iter()
                .map(|sphere| Hittable::new(GeometryType::Sphere(*sphere)))
                .collect(),
        }
    }

    pub fn get_hittables(&self) -> &Vec<Hittable> {
        &self.hittables
    }
}
