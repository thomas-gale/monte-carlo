use std::cmp::Ordering;
use std::collections::VecDeque;

use super::aabb::surrounding_box;
use super::bvh_node::BvhNode;
use super::linear_hittable::{GeometryType, LinearHittable};
use super::linear_scene_bvh::LinearSceneBvh;

use super::util;

///
/// Box reference based bvh node, used for recursive bvh construction
///
#[derive(Debug, Clone)]
pub struct SceneBvhConstructionNode {
    left: Option<Box<SceneBvhConstructionNode>>,
    right: Option<Box<SceneBvhConstructionNode>>,
    hittable: LinearHittable,
}

impl SceneBvhConstructionNode {
    fn leaf(hittable: LinearHittable) -> Self {
        SceneBvhConstructionNode {
            left: None,
            right: None,
            hittable,
        }
    }

    ///
    /// Recursive constructor
    /// https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/hierarchiesofboundingvolumes
    ///
    pub fn new(source_objects: &[LinearHittable]) -> Self {
        let mut objects = source_objects.to_vec();

        // Compute random sorting axis (for X, Y, Z)
        // let axis = util::random_int(0, 2) as usize;

        // Hard code to XZ (the horizontal plane)
        let mut axis = util::random_int(0, 1) as usize;
        if axis == 1 {
            axis = 2;
        }

        // Hard code to X
        // let axis = 0 as usize;
        // let comparator = |a, b| -> Ordering { box_compare(a, b, axis) };

        // The nodes
        let left: Option<Box<SceneBvhConstructionNode>>;
        let right: Option<Box<SceneBvhConstructionNode>>;

        // If we have only 1 or 2 items to place in bvh (base cases)
        if source_objects.len() == 1 {
            left = Some(Box::new(SceneBvhConstructionNode::leaf(objects[0])));
            right = left.clone();
        } else if source_objects.len() == 2 {
            if box_compare(&objects[0], &objects[1], axis) == Ordering::Less {
                left = Some(Box::new(SceneBvhConstructionNode::leaf(objects[0])));
                right = Some(Box::new(SceneBvhConstructionNode::leaf(objects[1])));
            } else {
                left = Some(Box::new(SceneBvhConstructionNode::leaf(objects[1])));
                right = Some(Box::new(SceneBvhConstructionNode::leaf(objects[0])));
            }
        } else {
            // General recursive case
            objects.sort_by(|a, b| box_compare(a, b, axis));
            let mid = objects.len() / 2;
            left = Some(Box::new(SceneBvhConstructionNode::new(&objects[0..mid])));
            right = Some(Box::new(SceneBvhConstructionNode::new(&objects[mid..])))
        }

        let box_left = left.as_ref().unwrap().hittable.bounding_box();
        let box_right = right.as_ref().unwrap().hittable.bounding_box();

        let box_surround = surrounding_box(&box_left, &box_right);

        SceneBvhConstructionNode {
            left,
            right,
            hittable: LinearHittable::new(GeometryType::BvhNode(BvhNode::new(0, 0, box_surround))),
        }
    }

    ///
    /// Convert the box based referential structure into a flat (linearized version) of the Bvh, using the POD BvhNode data structure that uses index
    /// based referencing to child nodes
    ///  
    pub fn flatten(&self) -> LinearSceneBvh {
        // Bvh construction flattened.
        let mut flat_bvh_hittables: Vec<LinearHittable> = vec![];

        // BFS traversal
        let mut queue: VecDeque<Box<SceneBvhConstructionNode>> = VecDeque::new();
        queue.push_back(Box::new(self.clone()));

        while !queue.is_empty() {
            let current = queue.pop_front();
            let current_ref = current.as_ref().unwrap();

            // Create a flattened hittable
            let mut flat_hittable = current_ref.hittable.clone();

            if current_ref.left.is_some() {
                // Add the left child to the bfs queue to process later
                queue.push_back(current_ref.left.clone().unwrap());
                // Add the computed index of left child (which will be added later)
                flat_hittable
                    .bvh_node
                    .set_left((flat_bvh_hittables.len() + queue.len()) as u32)
            }
            if current_ref.right.is_some() {
                // Add the right child to the bfs queue to process later
                queue.push_back(current_ref.right.clone().unwrap());
                // Add the computed index of right child (which will be added later)
                flat_hittable
                    .bvh_node
                    .set_right((flat_bvh_hittables.len() + queue.len()) as u32)
            }

            // Add the flattened hittable to the collection
            flat_bvh_hittables.push(flat_hittable);
        }

        // Debug - pretty print the flattened scene bvh
        // for hittable in flat_bvh_hittables.iter() {
        //     if hittable.geometry_type == 0 {
        //         println!("\n BVH Node: {:?}", hittable.bvh_node);
        //     } else if hittable.geometry_type == 1 {
        //         println!("\n Sphere: {:?}", hittable.sphere);
        //     }
        // }

        LinearSceneBvh::build_from_hittables(flat_bvh_hittables)
    }
}

fn box_compare(a: &LinearHittable, b: &LinearHittable, axis: usize) -> Ordering {
    a.bounding_box().min()[axis]
        .partial_cmp(&b.bounding_box().max()[axis])
        .unwrap()
}
