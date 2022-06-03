use super::aabb::Aabb;
use super::hittable::{self, Hittable};

use super::util;

///
/// Box reference based bvh node, used for recursive bvh construction
///
#[derive(Debug, Clone)]
pub struct BvhConstructionNode {
    left: Option<Box<BvhConstructionNode>>,
    right: Option<Box<BvhConstructionNode>>,
    // aabb: Aabb,
    hittable: Hittable,
}

impl BvhConstructionNode {
    fn leaf(hittable: Hittable) -> Self {
        BvhConstructionNode {
            left: None,
            right: None,
            hittable,
        }
    }

    ///
    /// Recursive constructor
    /// https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/hierarchiesofboundingvolumes
    ///
    pub fn new(source_objects: &[Hittable]) -> Self {
        let mut objects = source_objects.to_vec();

        // Compute random sorting axis
        let axis = util::random_int(0, 2) as usize;
        let comparitor = |a, b| -> bool { box_compare(a, b, axis) };

        // The nodes
        let mut left: Option<Box<BvhConstructionNode>> = None;
        let mut right: Option<Box<BvhConstructionNode>> = None;

        // If we have only 1 or 2 items to place in bvh
        if source_objects.len() == 1 {
            left = Some(Box::new(BvhConstructionNode::leaf(objects[0])));
            right = left.clone();
        } else if (source_objects.len() == 2) {
            if (comparitor(&objects[0], &objects[1])) {
                left = Some(Box::new(BvhConstructionNode::leaf(objects[0])));
                right = Some(Box::new(BvhConstructionNode::leaf(objects[1])));
            } else {
                left = Some(Box::new(BvhConstructionNode::leaf(objects[1])));
                right = Some(Box::new(BvhConstructionNode::leaf(objects[0])));
            }
        } else {
					// General recursive case
					

				}

        BvhConstructionNode {
            left: None,
            right: None,
            // aabb: Aabb::empty(),
            hittable: Hittable::empty(),
        }
    }
}

fn box_compare(a: &Hittable, b: &Hittable, axis: usize) -> bool {
    a.bounding_box().min()[axis] < b.bounding_box().max()[axis]
}

fn box_x_compare(a: &Hittable, b: &Hittable) -> bool {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Hittable, b: &Hittable) -> bool {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Hittable, b: &Hittable) -> bool {
    box_compare(a, b, 2)
}
