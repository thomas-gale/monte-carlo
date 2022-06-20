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
    /// Fraction of light attenuated by each bvh traversed - bit hacky (larger scenes will need values like 0.999 and small scenes 0.9)
    draw_bvh_attenuation: f32,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            infinity: f32::INFINITY,
            epsilon: 1.0e-4,
            pi: std::f32::consts::PI,
            pass_samples_per_pixel: 1,
            max_depth: 64,
            vertical_render_slices: 1,
            draw_vertical_render_slice_region: 0,
            draw_bvh: 0,
            draw_bvh_attenuation: 0.8,
        }
    }
}
