use crate::bvh_raytracing::bvh;

use super::{
    bvh_construction_node::BvhConstructionNode,
    hittable::{self, *},
    sphere::Sphere,
};

///
/// The basic linearized version of the scene, ready to be transferred to the GPU
///
pub struct Bvh {
    hittables: Vec<Hittable>,
}

impl Bvh {
    /// Experimental function to build a BVH from a slice of spheres
    pub fn build_from_spheres(spheres: &[Sphere]) -> Self {
        let hittables: Vec<Hittable> = spheres
            .iter()
            .map(|sphere| Hittable::new(GeometryType::Sphere(*sphere)))
            .collect();

        // WIP
        let bvh_construction = BvhConstructionNode::new(&hittables[..]);

        println!("{:?}", bvh_construction);

        Bvh { hittables }
    }

    pub fn get_hittables(&self) -> &Vec<Hittable> {
        &self.hittables
    }
}
