use cgmath::Point3;

use super::{aabb::Aabb, linear_scene_bvh::LinearSceneBvh};

/// POD plain triangle vertex ready to ship to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleVertex {
    pub position: [f32; 3],
    _pad1: u32,
}

impl TriangleVertex {
    pub fn new(position: [f32; 3]) -> Self {
        TriangleVertex { position, _pad1: 0 }
    }
    pub fn empty() -> Self {
        TriangleVertex {
            position: [0.0, 0.0, 0.0],
            _pad1: 0,
        }
    }
}

/// POD plain triangle (references indices in the triangle vertex buffer in the scene) ready to ship to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Triangle {
    /// This is the scene index in the Triangle Vertex buffer/array
    pub indices: [u32; 3],
    _pad1: u32,
}

impl Triangle {
    pub fn new(indices: [u32; 3]) -> Self {
        Triangle { indices, _pad1: 0 }
    }
    pub fn empty() -> Self {
        Triangle {
            indices: [0, 0, 0],
            _pad1: 0,
        }
    }
    /// Returns the bounding box of the constant medium
    pub fn bounding_box(&self, scene: &LinearSceneBvh) -> Aabb {
        // Grab the triangle vertices from the scene and compute a Aabb
        let v_1: Point3<f32> = scene.tri_verts[self.indices[0] as usize].position.into();
        let v_2: Point3<f32> = scene.tri_verts[self.indices[1] as usize].position.into();
        let v_3: Point3<f32> = scene.tri_verts[self.indices[2] as usize].position.into();

        Aabb::new(
            Point3::new(
                v_1.x.min(v_2.x).min(v_3.x),
                v_1.y.min(v_2.y).min(v_3.y),
                v_1.z.min(v_2.z).min(v_3.z),
            ),
            Point3::new(
                v_1.x.max(v_2.x).max(v_3.x),
                v_1.y.max(v_2.y).max(v_3.y),
                v_1.z.max(v_2.z).max(v_3.z),
            ),
        )
    }
}
