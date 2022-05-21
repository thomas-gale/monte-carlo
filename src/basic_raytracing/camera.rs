use wgpu::util::DeviceExt;

// Note: Due to wgsl uniforms requiring 16 byte (4 float) spacing, we need to use a padding fields here.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub origin: [f32; 3],
    _padding1: f32,
    pub lower_left_corner: [f32; 3],
    _padding2: f32,
    pub horizontal: [f32; 3],
    _padding3: f32,
    pub vertical: [f32; 3],
    _padding4: f32,
}

impl Camera {
    // TODO - take viewport information.
    pub fn new() -> Camera {
        Camera {
            origin: [0.0, 0.0, 0.0],
            _padding1: 0.0,
            lower_left_corner: [-0.5, -0.5, 1.0],
            _padding2: 0.0,
            horizontal: [1.0, 0.0, 0.0],
            _padding3: 0.0,
            vertical: [0.0, 1.0, 0.0],
            _padding4: 0.0,
        }
    }

    // TODO - check if passing ownership of camera is a good thing?!.
    pub fn to_device_buffer(self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[self]),
            usage: wgpu::BufferUsages::UNIFORM,
        })
    }

    // TODO - passed the buffer back into this function seems clunky
    pub fn create_binding(
        &self,
        buffer: &wgpu::Buffer,
        device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });
        (bind_group_layout, bind_group)
    }

    // Layout description
    //  pub fn get_layout_description<'a>() -> wgpu::VertexBufferLayout<'a> {
    //       wgpu::VertexBufferLayout {
    //           array_stride: std::mem::size_of::<Camera>() as wgpu::BufferAddress,
    //           step_mode: wgpu::VertexStepMode::Vertex,
    //           attributes: &[
    //               wgpu::VertexAttribute {
    //                   offset: 0,
    //                   shader_location: 0,
    //                   format: wgpu::VertexFormat::Float32x3, // position
    //               },
    //               wgpu::VertexAttribute {
    //                   offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
    //                   shader_location: 1,
    //                   format: wgpu::VertexFormat::Float32x2, // tex_coords
    //               },
    //           ],
    //       }
    //   }
}
