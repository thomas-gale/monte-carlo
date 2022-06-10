#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Constants {
    infinity: f32,
    epsilon: f32,
    pi: f32,
    pass_samples_per_pixel: i32,
    /// Maximum depth of bounced ray.
    max_depth: i32,
    /// Number of vertical subdivision for single frame passes.
    vertical_render_slices: i32,
    /// 0: Off, 1: On
    draw_vertical_render_slice_region: u32,
    /// 0: Off, 1: On
    draw_bvh: u32,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            infinity: f32::INFINITY,
            epsilon: 1.0e-4,
            pi: std::f32::consts::PI,
            pass_samples_per_pixel: 1,
            max_depth: 64,
            vertical_render_slices: 32,
            draw_vertical_render_slice_region: 0,
            draw_bvh: 0,
        }
    }
}
