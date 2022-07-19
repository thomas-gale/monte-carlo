use super::{constant_medium::ConstantMedium, cuboid::Cuboid, mesh::Mesh, sphere::Sphere};

pub enum HittablePrimitive {
    // BvhNode(BvhNode),
    Sphere(Sphere),
    Cuboid(Cuboid),
    ConstantMedium(ConstantMedium),
    Mesh(Mesh),
}
