use super::aabb::Aabb;

///
/// POD BvhNode ready to ship to GPU
/// 
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BvhNode {
    /// Pointer to left hittable (u32 max == null)
    left_hittable: u32,
    /// Pointer to right hittable (u32 max == null)
    right_hittable: u32,
    _pad_1: u32,
    _pad_2: u32,
    aabb: Aabb,
}

impl BvhNode {
    ///
    /// Create a new BvhNode
    /// TODO - the pointer for left and right should refer to some index in of slice/block of memory allocated for use on CPU->GPU
    ///
    pub fn new(left_hittable: u32, right_hittable: u32, aabb: Aabb) -> Self {
        BvhNode {
            left_hittable,
            right_hittable,
            _pad_1: 0,
            _pad_2: 0,
            aabb,
        }
    }

    pub fn empty() -> Self {
        BvhNode {
            left_hittable: u32::max_value(),
            right_hittable: u32::max_value(),
            _pad_1: 0,
            _pad_2: 0,
            aabb: Aabb::empty(),
        }
    }

    pub fn set_left(&mut self, left: u32) {
        self.left_hittable = left;
    }

    pub fn set_right(&mut self, right: u32) {
        self.right_hittable = right;
    }

    pub fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
