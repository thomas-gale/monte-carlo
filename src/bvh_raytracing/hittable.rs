use super::{bvh_node::BvhNode, sphere::Sphere};

#[derive(Copy, Clone, Debug)]
pub enum GeometryType {
    BvhNode(BvhNode),
    Sphere(Sphere),
}

///
/// Experimental data structure to hold all bvh compatible data for a single hittable geometry to compose into the bvh tree
///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Hittable {
    pub geometry_type: u32,
    pub _pad_1: u32,
    pub _pad_2: u32,
    pub _pad_3: u32,
    pub bvh_node: BvhNode,
    pub sphere: Sphere,
}

impl Hittable {
    ///
    /// Create a new hittable (which is a bytemuck::Pod and can be sent to GPU/addresses as a struct in wgsl)
    ///
    pub fn new(geometry_type: GeometryType) -> Self {
        match geometry_type {
            GeometryType::BvhNode(bvh_node) => Hittable {
                geometry_type: 0,
                _pad_1: 0,
                _pad_2: 0,
                _pad_3: 0,
                bvh_node,
                sphere: Sphere::empty(),
            },
            GeometryType::Sphere(sphere) => Hittable {
                geometry_type: 1,
                _pad_1: 0,
                _pad_2: 0,
                _pad_3: 0,
                bvh_node: BvhNode::empty(),
                sphere,
            },
        }
    }
}
