use cgmath::Vector3;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub material_type: u32,    // 0: lambertian, 1: metal, 2: dielectric
    pub fuzz: f32,             // Roughness for metals
    pub refraction_index: f32, // Refraction index for dielectrics
    pub _pad1: f32,
    pub albedo: [f32; 3], // Coloration of the ray bounce
    pub _pad2: f32,
}

impl Sphere {
    pub fn new(
        center: Vector3<f32>,
        radius: f32,
        material_type: u32,
        fuzz: f32,
        refraction_index: f32,
        albedo: Vector3<f32>,
    ) -> Self {
        Sphere {
            center: center.into(),
            radius,
            material_type,
            fuzz,
            refraction_index,
            _pad1: 0.0,
            albedo: albedo.into(),
            _pad2: 0.0,
        }
    }
}
