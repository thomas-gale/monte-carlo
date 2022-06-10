use super::{bvh_node::BvhNode, sphere::Sphere};

pub enum HittablePrimitive {
    BvhNode(BvhNode),
    Sphere(Sphere),
}
