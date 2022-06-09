///
/// POD Material ready to ship to GPU
///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    /// 0: lambertian, 1: metal, 2: dielectric
    pub material_type: u32,
    /// Roughness for metals
    pub fuzz: f32,
    /// Refraction index for dielectrics
    pub refraction_index: f32,
    pub _pad1: f32,
    /// Coloration of the ray bounce
    pub albedo: [f32; 3],
    pub _pad2: f32,
}
