#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub albedo: [f32; 3],
    pub material_type: u32, // 0: lambertian, 1: metal, 2: dielectric
}
