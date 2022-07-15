use super::{bvh_node::BvhNode, constant_medium::ConstantMedium, cuboid::Cuboid, sphere::Sphere, mesh::Mesh};

pub enum HittablePrimitive {
    BvhNode(BvhNode),
    Sphere(Sphere),
    Cuboid(Cuboid),
    Mesh(Mesh),
    ConstantMedium(ConstantMedium),
}
