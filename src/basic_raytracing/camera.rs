// Note: Due to wgsl uniforms requiring 16 byte (4 float) spacing, we need to use a padding fields here.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub origin: [f32; 3],
    _padding1: f32,
    pub lower_left_corner: [f32; 3],
    _padding2: f32,
    pub horizontal: [f32; 3],
    _padding3: f32,
    pub vertical: [f32; 3],
    _padding4: f32,
}

impl Camera {
    // TODO - take viewport information.
    pub fn new() -> Camera {
        Camera {
            origin: [0.0, 0.0, 0.0],
            _padding1: 0.0,
            lower_left_corner: [-0.5, -0.5, 1.0],
            _padding2: 0.0,
            horizontal: [1.0, 0.0, 0.0],
            _padding3: 0.0,
            vertical: [0.0, 1.0, 0.0],
            _padding4: 0.0,
        }
    }
}
