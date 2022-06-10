use super::{linear_hittable::*, scene_bvh_construction_node::SceneBvhConstructionNode, sphere::Sphere};

///
/// The basic linearized version of the scene, ready to be transferred to the GPU
///
#[derive(Debug)]
pub struct LinearSceneBvh {
    hittables: Vec<LinearHittable>,
}

impl LinearSceneBvh {
    /// Experimental function to build a BVH from a slice of spheres
    pub fn build_from_spheres(spheres: &[Sphere]) -> Self {
        let hittables: Vec<LinearHittable> = spheres
            .iter()
            .map(|sphere| LinearHittable::new(GeometryType::Sphere(*sphere)))
            .collect();

        let bvh_construction = SceneBvhConstructionNode::new(&hittables[..]);
        bvh_construction.flatten()
    }

    pub fn build_from_hittables(hittables: Vec<LinearHittable>) -> Self {
        LinearSceneBvh { hittables }
    }

    pub fn get_hittables(&self) -> &Vec<LinearHittable> {
        &self.hittables
    }
}
