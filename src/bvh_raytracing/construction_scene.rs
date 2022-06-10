use super::{
    bvh_node::BvhNode, construction_scene_bvh_node::SceneBvhConstructionNode, cuboid::Cuboid,
    hittable_primitive::HittablePrimitive, linear_hittable::LinearHittable,
    linear_scene_bvh::LinearSceneBvh, material::Material, sphere::Sphere,
};

pub struct ConstructionScene {
    root_bvh_node: Vec<BvhNode>,
    materials: Vec<Material>,
}

impl ConstructionScene {
    /// TODO - this is being replaced with direct construction from the "Construction Scene" methods
    /// Experimental function to build a BVH from a slice of spheres
    pub fn build_from_spheres(spheres: &[Sphere]) -> LinearSceneBvh {
        let hittables: Vec<LinearHittable> = spheres
            .iter()
            .map(|sphere| LinearHittable::new(HittablePrimitive::Sphere(*sphere)))
            .collect();

        let bvh_construction = SceneBvhConstructionNode::new(&hittables[..]);
        bvh_construction.flatten()
    }

    /// TODO - this is being replaced with direct construction from the "Construction Scene" methods
    pub fn build_from_hittables(hittables: Vec<LinearHittable>) -> LinearSceneBvh {
        LinearSceneBvh {
            hittables,
            bvh_nodes: vec![BvhNode::empty()],
            spheres: vec![Sphere::empty()],
            cuboids: vec![Cuboid::empty()],
            materials: vec![Material::empty()],
        }
    }
}
