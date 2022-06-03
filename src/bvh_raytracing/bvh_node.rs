use super::aabb::Aabb;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BvhNode {
    left_hittable: u32,
    /// Pointer to left hittable (u32 max == null)
    right_hittable: u32,
    /// Pointer to right hittable (u32 max == null)
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
            left_hittable: 0,
            right_hittable: 0,
            _pad_1: 0,
            _pad_2: 0,
            aabb: Aabb::empty(),
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
