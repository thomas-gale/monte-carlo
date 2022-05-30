use cgmath::Vector3;

///
/// Axis aligned bounding box
///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Aabb {
    min: [f32; 3],
    _padding1: f32,
    max: [f32; 3],
    _padding2: f32,
}

impl Aabb {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Aabb {
            min: min.into(),
            _padding1: 0.0,
            max: max.into(),
            _padding2: 0.0,
        }
    }
}
