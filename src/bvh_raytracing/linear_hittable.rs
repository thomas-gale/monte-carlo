use super::{aabb::Aabb, bvh_node::BvhNode, sphere::Sphere};

pub enum GeometryType {
    BvhNode(BvhNode),
    Sphere(Sphere),
}

///
/// Experimental data structure to hold all bvh compatible data for a single hittable geometry to compose into the bvh tree
/// This is the linearized form, expected to be part of the linear scene bvh
///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LinearHittable {
    /// 0 = BvhNode, 1 = Sphere
    pub geometry_type: u32,
    pub _pad_1: u32,
    pub _pad_2: u32,
    pub _pad_3: u32,
    pub bvh_node: BvhNode,
    pub sphere: Sphere,
}

impl LinearHittable {
    ///
    /// Create a new hittable (which is a bytemuck::Pod and can be sent to GPU/addresses as a struct in wgsl)
    ///
    pub fn new(geometry_type: GeometryType) -> Self {
        match geometry_type {
            GeometryType::BvhNode(bvh_node) => LinearHittable {
                geometry_type: 0,
                _pad_1: 0,
                _pad_2: 0,
                _pad_3: 0,
                bvh_node,
                sphere: Sphere::empty(),
            },
            GeometryType::Sphere(sphere) => LinearHittable {
                geometry_type: 1,
                _pad_1: 0,
                _pad_2: 0,
                _pad_3: 0,
                bvh_node: BvhNode::empty(),
                sphere,
            },
        }
    }

    ///
    /// Empty hittable
    ///
    // pub fn empty() -> Self {
    //     Hittable {
    //         geometry_type: u32::max_value(),
    //         _pad_1: 0,
    //         _pad_2: 0,
    //         _pad_3: 0,
    //         bvh_node: BvhNode::empty(),
    //         sphere: Sphere::empty(),
    //     }
    // }

    pub fn bounding_box(&self) -> Aabb {
        match self.geometry_type {
            0 => self.bvh_node.bounding_box(),
            1 => self.sphere.bounding_box(),
            _ => Aabb::empty(),
        }
    }
}
