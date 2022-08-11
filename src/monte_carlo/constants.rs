use cgmath::Vector3;

use super::material::Material;

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
    /// WoS Tolerance Distance (e.g. distance to surface before the walk is halted and surface sampled)
    wos_tolerance: f32,
    _pad1: u32,
    _pad2: u32,
    /// Background color
    background: Material,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            infinity: f32::INFINITY,
            epsilon: 1.0e-5,
            pi: std::f32::consts::PI,
            pass_samples_per_pixel: 1,
            max_depth: 64,
            vertical_render_slices: 64,
            draw_vertical_render_slice_region: 0,
            draw_bvh: 0,
            draw_bvh_attenuation: 0.8,
            wos_tolerance: 0.005,
            _pad1: 0,
            _pad2: 0,
            background: Material::new(0, 0.0, 0.0, Vector3::new(0.70, 0.80, 1.00)),
        }
    }
}
