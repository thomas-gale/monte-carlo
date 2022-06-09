#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Constants {
    infinity: f32,
    epsilon: f32,
    pi: f32,
    pass_samples_per_pixel: i32,
    max_depth: i32,
    render_patch_sub_divisions: i32,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            infinity: f32::INFINITY,
            epsilon: 1.0e-4,
            pi: std::f32::consts::PI,
            pass_samples_per_pixel: 1,
            max_depth: 50,
            render_patch_sub_divisions: 16,
        }
    }
}
