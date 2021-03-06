use cgmath::Point3;

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
    ///
    /// Construct a new Axis aligned bounding box from cgmath Point3s
    ///
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        Aabb {
            min: min.into(),
            _padding1: 0.0,
            max: max.into(),
            _padding2: 0.0,
        }
    }

    pub fn empty() -> Self {
        Aabb {
            min: [0.0; 3],
            _padding1: 0.0,
            max: [0.0; 3],
            _padding2: 0.0,
        }
    }

    pub fn min(&self) -> &[f32; 3] {
        &self.min
    }

    pub fn max(&self) -> &[f32; 3] {
        &self.max
    }
}

///
/// Compute the bounding box of two bounding boxes
///
pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Point3::new(
        box0.min[0].min(box1.min[0]),
        box0.min[1].min(box1.min[1]),
        box0.min[2].min(box1.min[2]),
    );
    let big = Point3::new(
        box0.max[0].max(box1.max[0]),
        box0.max[1].max(box1.max[1]),
        box0.max[2].max(box1.max[2]),
    );
    Aabb::new(small, big)
}
