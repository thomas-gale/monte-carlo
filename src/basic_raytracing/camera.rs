use cgmath::Vector3;

use super::buffer_bindings;

// Note: Due to wgsl uniforms requiring 16 byte (4 float) spacing, we need to use a padding fields here.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraRaw {
    pub origin: [f32; 3],
    _padding1: f32,
    pub lower_left_corner: [f32; 3],
    _padding2: f32,
    pub horizontal: [f32; 3],
    _padding3: f32,
    pub vertical: [f32; 3],
    _padding4: f32,
}

impl CameraRaw {
    // TODO - take viewport information.
    pub fn new() -> CameraRaw {
        CameraRaw {
            origin: [0.0, 0.0, 0.0],
            _padding1: 0.0,
            lower_left_corner: [-1.0, -1.0, -1.0],
            _padding2: 0.0,
            horizontal: [2.0, 0.0, 0.0],
            _padding3: 0.0,
            vertical: [0.0, 2.0, 0.0],
            _padding4: 0.0,
        }
    }
}

pub struct Camera {
    raw: CameraRaw,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Self {
        let raw = CameraRaw::new();
        let (bind_group_layout, bind_group, buffer) = buffer_bindings::create_device_buffer_binding(
            &[raw],
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Uniform,
        );

        Camera {
            raw: CameraRaw::new(),
            bind_group_layout,
            bind_group,
            buffer,
        }
    }

    pub fn translate(&mut self, queue: &wgpu::Queue, delta: Vector3<f32>) {
        self.raw.origin[0] += delta.x;
        self.raw.origin[1] += delta.y;
        self.raw.origin[2] += delta.z;
        self.raw.lower_left_corner[0] += delta.x;
        self.raw.lower_left_corner[1] += delta.y;
        self.raw.lower_left_corner[2] += delta.z;
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
