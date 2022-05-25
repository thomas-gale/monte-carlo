use wgpu::util::DeviceExt;

pub fn create_device_buffer_binding<T: bytemuck::Pod>(
    entity_slice: &[T],
    device: &wgpu::Device,
    usage: wgpu::BufferUsages,
    binding_type: wgpu::BufferBindingType,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup, wgpu::Buffer) {
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(entity_slice),
        usage,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            count: None,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: binding_type,
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
    (bind_group_layout, bind_group, buffer)
}
