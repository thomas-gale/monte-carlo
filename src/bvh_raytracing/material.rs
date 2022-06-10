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

impl Material {
    pub fn empty() -> Self {
        Material {
            material_type: 0,
            fuzz: 0.0,
            refraction_index: 0.0,
            _pad1: 0.0,
            albedo: [0.0; 3],
            _pad2: 0.0,
        }
    }
}
