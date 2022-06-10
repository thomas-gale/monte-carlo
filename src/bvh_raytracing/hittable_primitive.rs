use super::{bvh_node::BvhNode, cuboid::Cuboid, sphere::Sphere};

pub enum HittablePrimitive {
    BvhNode(BvhNode),
    Sphere(Sphere),
    Cuboid(Cuboid),
}
