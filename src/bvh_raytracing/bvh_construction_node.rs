use std::cmp::Ordering;
use std::collections::VecDeque;

use super::aabb::{surrounding_box, Aabb};
use super::bvh::Bvh;
use super::bvh_node::BvhNode;
use super::hittable::{self, GeometryType, Hittable};

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
        // let axis = util::random_int(0, 2) as usize;
        let axis = 0 as usize;
        // let comparator = |a, b| -> Ordering { box_compare(a, b, axis) };

        // The nodes
        let mut left: Option<Box<BvhConstructionNode>> = None;
        let mut right: Option<Box<BvhConstructionNode>> = None;

        // If we have only 1 or 2 items to place in bvh (base cases)
        if source_objects.len() == 1 {
            left = Some(Box::new(BvhConstructionNode::leaf(objects[0])));
            right = left.clone();
        } else if source_objects.len() == 2 {
            if box_compare(&objects[0], &objects[1], axis) == Ordering::Less {
                left = Some(Box::new(BvhConstructionNode::leaf(objects[0])));
                right = Some(Box::new(BvhConstructionNode::leaf(objects[1])));
            } else {
                left = Some(Box::new(BvhConstructionNode::leaf(objects[1])));
                right = Some(Box::new(BvhConstructionNode::leaf(objects[0])));
            }
        } else {
            // General recursive case
            objects.sort_by(|a, b| box_compare(a, b, axis));
            let mid = objects.len() / 2;
            left = Some(Box::new(BvhConstructionNode::new(&objects[0..mid])));
            right = Some(Box::new(BvhConstructionNode::new(&objects[mid..])))
        }

        let box_left = left.as_ref().unwrap().hittable.bounding_box();
        let box_right = right.as_ref().unwrap().hittable.bounding_box();

        let box_surround = surrounding_box(&box_left, &box_right);

        BvhConstructionNode {
            left,
            right,
            // aabb: Aabb::empty()
            // TODO - check the Bvh node is correct,
            hittable: Hittable::new(GeometryType::BvhNode(BvhNode::new(0, 0, box_surround))),
        }
    }

    ///
    /// Convert the box based referential structure into a flat (linearised version) of the Bvh, using the POD BvhNode data structure that uses index
    /// based referencing to child nodes
    ///  
    pub fn flatten(&self) -> Bvh {
        // Bvh construction flattened.
        // let mut flat_bvh_construction: Vec<BvhConstructionNode> = vec![];

        // BFS traversal
        let mut queue: VecDeque<Box<BvhConstructionNode>> = VecDeque::new();
        queue.push_back(Box::new(self.clone()));

        while !queue.is_empty() {
            let current = queue.pop_front();
            let current_ref = current.as_ref().unwrap();

            println!("\n TEST Current: {:?}", current_ref);
            // hittables.push(current_ref.hittable);

            if current_ref.left.is_some() {
                queue.push_back(current_ref.left.clone().unwrap());
            }
            if current_ref.right.is_some() {
                queue.push_back(current_ref.right.clone().unwrap());
            }
        }

        Bvh::build_from_hittables(vec![])
    }
}

fn box_compare(a: &Hittable, b: &Hittable, axis: usize) -> Ordering {
    a.bounding_box().min()[axis]
        .partial_cmp(&b.bounding_box().max()[axis])
        .unwrap()
}

// fn box_x_compare(a: &Hittable, b: &Hittable) -> bool {
//     box_compare(a, b, 0)
// }

// fn box_y_compare(a: &Hittable, b: &Hittable) -> bool {
//     box_compare(a, b, 1)
// }

// fn box_z_compare(a: &Hittable, b: &Hittable) -> bool {
//     box_compare(a, b, 2)
// }
