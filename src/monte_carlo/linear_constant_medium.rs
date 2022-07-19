use super::{aabb::Aabb, linear_scene_bvh::LinearSceneBvh};

///
/// POD ConstantMedium ready to ship to GPU
///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LinearConstantMedium {
    /// 0: BvhNode, 1: Sphere, 2: Cuboid, 3: ConstantMedium
    pub boundary_geometry_type: u32,
    /// Given the geometry type, the actual data is stored at the following index in the linear_scene_bvh vector (for the appropriate type).
    pub boundary_scene_index: u32,
    /// Index of the material in the linear scene bvh (know as phase function)
    pub material_index: u32,
    /// Negative inverse of the density of the medium
    pub neg_inv_density: f32,
}

impl LinearConstantMedium {
    ///
    /// Construct a new constant medium
    /// * `boundary_geometry_type` - 0: BvhNode, 1: Sphere, 2: Cuboid, 3: ConstantMedium
    /// * `boundary_scene_index` - Given the geometry type, the actual data is stored at the following index in the linear_scene_bvh vector (for the appropriate type).
    /// * `material_index` - Index of the material in the linear scene bvh
    /// * `density` - Density of this medium
    pub fn new(
        boundary_geometry_type: u32,
        boundary_scene_index: u32,
        material_index: u32,
        density: f32,
    ) -> Self {
        LinearConstantMedium {
            boundary_geometry_type,
            boundary_scene_index,
            material_index,
            neg_inv_density: -1.0 / density,
        }
    }

    pub fn empty() -> Self {
        LinearConstantMedium {
            boundary_geometry_type: LinearSceneBvh::null_index_ptr(),
            boundary_scene_index: LinearSceneBvh::null_index_ptr(),
            material_index: LinearSceneBvh::null_index_ptr(),
            neg_inv_density: 0.0,
        }
    }

    /// Returns the bounding box of the constant medium
    pub fn bounding_box(&self, scene: &LinearSceneBvh) -> Aabb {
        match self.boundary_geometry_type {
            // Sphere
            1 => scene.spheres[self.boundary_scene_index as usize].bounding_box(),
            // Cuboid
            2 => scene.cuboids[self.boundary_scene_index as usize].bounding_box(),
            _ => panic!("Unsupported geometry type"),
        }
    }
}
