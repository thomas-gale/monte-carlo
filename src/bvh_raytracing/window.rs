#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Window {
    pub width_pixels: u32,
    pub height_pixels: u32,
}

impl Window {
    pub fn new(size: &winit::dpi::PhysicalSize<u32>) -> Self {
        Window {
            width_pixels: size.width,
            height_pixels: size.height,
        }
    }
}
