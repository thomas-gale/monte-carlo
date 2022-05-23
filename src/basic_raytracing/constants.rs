#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Constants {
    infinity: f32,
    pi: f32,
    samples_per_pixel: i32,
    max_depth: i32,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            infinity: std::f32::INFINITY,
            pi: std::f32::consts::PI,
            samples_per_pixel: 1,
            max_depth: 5,
        }
    }
}
