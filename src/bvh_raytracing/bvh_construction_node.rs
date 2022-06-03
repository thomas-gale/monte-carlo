use super::aabb::Aabb;
use super::hittable::Hittable;

///
/// Box reference based bvh node, used for recursive bvh construction
///
#[derive(Debug)]
pub struct BvhConstructionNode {
    left: Option<Box<BvhConstructionNode>>,
    right: Option<Box<BvhConstructionNode>>,
    aabb: Aabb,
    hittable: Hittable,
}

impl BvhConstructionNode {
    ///
    /// Recursive constructor
    /// https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/hierarchiesofboundingvolumes
    ///
    pub fn new(source_objects: &[Hittable]) -> Self {
        let mut objects = source_objects.to_vec();

        BvhConstructionNode {
            left: None,
            right: None,
            aabb: Aabb::empty(),
            hittable: Hittable::empty(),
        }
    }
}
