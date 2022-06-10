use super::{bvh_node::BvhNode, material::Material};


#[derive(Debug, Clone)]
pub struct ConstructionScene {
    root_bvh_node: Vec<BvhNode>,
    materials: Vec<Material>,
}
