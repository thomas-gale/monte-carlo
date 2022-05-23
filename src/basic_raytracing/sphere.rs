#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub _pad1: f32,         // Byte 16
    pub albedo: [f32; 3],   // Coloration of the ray bounce
    pub material_type: u32, // 0: lambertian, 1: metal, 2: dielectric
    pub radius: f32,
    pub fuzz: f32,  // Roughness for metals
    pub _pad2: f32, 
    pub _pad3: f32, // Byte 48
}
