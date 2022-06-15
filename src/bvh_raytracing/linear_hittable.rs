use super::{
    aabb::Aabb,
    linear_scene_bvh::LinearSceneBvh,
};

///
/// Experimental data structure to hold all bvh compatible data for a single hittable geometry to compose into the bvh tree
/// This is the linearized form, expected to be part of the linear scene bvh
///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LinearHittable {
    /// 0: BvhNode, 1: Sphere, 2: Cuboid
    pub geometry_type: u32,
    /// Given the geometry type, the actual data is stored at the following index in the linear_scene_bvh vector (for the appropriate type).
    pub scene_index: u32,
}

impl LinearHittable {
    pub fn get_scene_index(&self) -> usize {
        self.scene_index as usize
    }

    /// Find the bounding box, looking up the underlying data from the scene
    pub fn bounding_box(&self, scene: &LinearSceneBvh) -> Aabb {
        // This will change to requiring a reference to the scenes current state so we can read from the linearized scene array.
        match self.geometry_type {
            // BvhNode
            0 => scene.bvh_nodes[self.get_scene_index()].bounding_box(),
            // Sphere
            1 => scene.spheres[self.get_scene_index()].bounding_box(),
            // Cuboid & others (TODO)
            _ => Aabb::empty(),
        }
    }
}
