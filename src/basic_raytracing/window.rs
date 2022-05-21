use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Window {
    width_pixels: u32,
    height_pixels: u32,
}

impl Window {
    pub fn new(size: &PhysicalSize<u32>) -> Self {
        Window {
            width_pixels: size.width,
            height_pixels: size.height,
        }
    }
}
