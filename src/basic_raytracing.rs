mod buffer_bindings;
mod camera;
mod constants;
mod quad;
mod scene;
mod sphere;
mod vertex;
mod window;

use winit::window::Window;

// I don't like this massive blob.
pub struct BasicRaytracing {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    quad: quad::Quad,
    render_pipeline: wgpu::RenderPipeline,
    constants_bind_group: wgpu::BindGroup,
    window_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    scene_bind_group: wgpu::BindGroup,
}

impl BasicRaytracing {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // Create instance, adapter, surface, device, queue and configuration

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // Constants
        let constants = constants::Constants::new();
        let (constants_bind_group_layout, constants_bind_group) =
            buffer_bindings::create_device_buffer_binding(
                &[constants],
                &device,
                wgpu::BufferUsages::UNIFORM,
                wgpu::BufferBindingType::Uniform,
            );

        // Window
        let window = window::Window::new(&size);
        let (window_bind_group_layout, window_bind_group) =
            buffer_bindings::create_device_buffer_binding(
                &[window],
                &device,
                wgpu::BufferUsages::UNIFORM,
                wgpu::BufferBindingType::Uniform,
            );

        // Camera
        let camera = camera::Camera::new();
        let (camera_bind_group_layout, camera_bind_group) =
            buffer_bindings::create_device_buffer_binding(
                &[camera],
                &device,
                wgpu::BufferUsages::UNIFORM,
                wgpu::BufferBindingType::Uniform,
            );

        // Scene
        let scene = scene::Scene::new();
        let (scene_bind_group_layout, scene_bind_group) =
            buffer_bindings::create_device_buffer_binding(
                &scene.spheres[..],
                &device,
                wgpu::BufferUsages::STORAGE,
                wgpu::BufferBindingType::Storage { read_only: (true) },
            );

        // Create basic quad to render fragments onto.
        let quad = quad::Quad::new(&device);

        // Load shader
        let shader = device.create_shader_module(&wgpu::include_wgsl!("basic_raytracing.wgsl"));

        // Create the render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &constants_bind_group_layout,
                    &window_bind_group_layout,
                    &camera_bind_group_layout,
                    &scene_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex::Vertex::layout_description()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            quad,
            render_pipeline,
            constants_bind_group,
            window_bind_group,
            camera_bind_group,
            scene_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);

            let window = window::Window::new(&self.size);
            let (_, window_bind_group) = buffer_bindings::create_device_buffer_binding(
                &[window],
                &self.device,
                wgpu::BufferUsages::UNIFORM,
                wgpu::BufferBindingType::Uniform,
            );
            self.window_bind_group = window_bind_group;
        }
    }

    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn update(&mut self) {
        // TODO
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.9,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.quad.vertices.slice(..));
            render_pass.set_index_buffer(self.quad.indices.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.constants_bind_group, &[]);
            render_pass.set_bind_group(1, &self.window_bind_group, &[]);
            render_pass.set_bind_group(2, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(3, &self.scene_bind_group, &[]);
            render_pass.draw_indexed(0..self.quad.num_indices, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
