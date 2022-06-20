use super::hittable_primitive::HittablePrimitive;

pub struct ConstantMedium {
    /// Enum reference to HittablePrimitive
    pub boundary_hittable: Box<HittablePrimitive>,
    /// Index of the material in the linear scene bvh (know as phase function)
    pub material_index: u32,
    /// Density of the medium
    pub density: f32,
}
