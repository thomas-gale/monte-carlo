use super::{bvh_node::BvhNode, constant_medium::ConstantMedium, cuboid::Cuboid, sphere::Sphere};

pub enum HittablePrimitive {
    BvhNode(BvhNode),
    Sphere(Sphere),
    Cuboid(Cuboid),
    ConstantMedium(ConstantMedium),
}
