use std::cmp::Ordering;
use std::collections::VecDeque;

use super::aabb::{surrounding_box, Aabb};
use super::bvh_node::BvhNode;
use super::linear_hittable::LinearHittable;
use super::linear_scene_bvh::LinearSceneBvh;
use super::util;

/// Box reference based bvh node, used for recursive bvh construction
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

    /// Recursive constructor
    /// https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/hierarchiesofboundingvolumes
    pub fn new(scene: &mut LinearSceneBvh, source_objects: &[LinearHittable]) -> Self {
        // Grab a vector to from the source objects - TODO check if we can remove this
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
        let mut right: Option<Box<SceneBvhConstructionNode>> = None;

        // If we have only 1 or 2 items to place in bvh (base cases)
        if objects.len() == 1 {
            // left arm of the tree is always used in the case of a single leaf.
            left = Some(Box::new(SceneBvhConstructionNode::leaf(objects[0])));
        } else if objects.len() == 2 {
            // Quick swap without recursion.
            if box_compare(scene, &objects[0], &objects[1], axis) == Ordering::Less {
                left = Some(Box::new(SceneBvhConstructionNode::leaf(objects[0])));
                right = Some(Box::new(SceneBvhConstructionNode::leaf(objects[1])));
            } else {
                left = Some(Box::new(SceneBvhConstructionNode::leaf(objects[1])));
                right = Some(Box::new(SceneBvhConstructionNode::leaf(objects[0])));
            }
        } else {
            // General recursive case
            objects.sort_by(|a, b| box_compare(scene, a, b, axis));
            let mid = objects.len() / 2;
            left = Some(Box::new(SceneBvhConstructionNode::new(
                scene,
                &objects[0..mid],
            )));
            right = Some(Box::new(SceneBvhConstructionNode::new(
                scene,
                &objects[mid..],
            )))
        }

        let box_left = left.as_ref().unwrap().hittable.bounding_box(scene);
        let box_surround: Aabb;
        if right.is_some() {
            let box_right = right.as_ref().unwrap().hittable.bounding_box(scene);
            box_surround = surrounding_box(&box_left, &box_right);
        } else {
            box_surround = box_left;
        }

        // In the case where we are creating a new bvh node primitive, we need to push this value to a bvh storage vector
        scene.bvh_nodes.push(BvhNode::new(
            BvhNode::null_hittable_ptr(),
            BvhNode::null_hittable_ptr(),
            box_surround,
        ));

        // Return a new bvh node with the reference to the new index in the linearized scene storage.
        SceneBvhConstructionNode {
            left,
            right,
            hittable: LinearHittable {
                geometry_type: 0,
                scene_index: (scene.bvh_nodes.len() - 1) as u32,
            },
        }
    }

    /// Convert the box based referential structure into a flat (linearized version) of the Bvh, using the POD BvhNode data structure that uses index
    /// based referencing to child nodes
    pub fn flatten(&self, scene: &mut LinearSceneBvh) {
        // Bvh construction flattened.
        let mut flat_bvh_hittables: Vec<LinearHittable> = vec![];

        // BFS traversal
        let mut queue: VecDeque<Box<SceneBvhConstructionNode>> = VecDeque::new();
        queue.push_back(Box::new(self.clone()));

        while !queue.is_empty() {
            let current = queue.pop_front();
            let current_ref = current.as_ref().unwrap();

            // Create a flattened hittable
            let flat_hittable = current_ref.hittable.clone();

            if current_ref.left.is_some() {
                // Add the left child to the bfs queue to process later
                queue.push_back(current_ref.left.clone().unwrap());
                // Add the computed index of left child (which will be added later)
                scene.bvh_nodes[flat_hittable.get_scene_index()]
                    .set_left((flat_bvh_hittables.len() + queue.len()) as u32);
            }
            if current_ref.right.is_some() {
                // Add the right child to the bfs queue to process later
                queue.push_back(current_ref.right.clone().unwrap());
                // Add the computed index of right child (which will be added later)
                scene.bvh_nodes[flat_hittable.get_scene_index()]
                    .set_right((flat_bvh_hittables.len() + queue.len()) as u32);
            }

            // Add the flattened hittable to the collection
            flat_bvh_hittables.push(flat_hittable);
        }

        // Finally, update the scene primitives
        scene.hittables = flat_bvh_hittables;
    }
}

/// Given a scene (containing the underlying scene data), compare a hittable entity to another hittable entity (using bounding box and sorting axis)
fn box_compare(
    scene: &LinearSceneBvh,
    a: &LinearHittable,
    b: &LinearHittable,
    axis: usize,
) -> Ordering {
    a.bounding_box(scene).min()[axis]
        .partial_cmp(&b.bounding_box(scene).max()[axis])
        .unwrap()
}
