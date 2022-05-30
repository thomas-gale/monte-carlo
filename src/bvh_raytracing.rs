mod aabb;
mod buffer_bindings;
mod camera;
mod constants;
mod quad;
mod result;
mod scene;
mod sphere;
mod uniforms_bindings;
mod util;
mod vertex;
mod window;

use cgmath::{Point3, Vector2, Vector3};
use winit::{event::WindowEvent, window::Window};

// Some bits need to be tidied into more granular structs.
pub struct BvhRaytracing {
    mouse_down: bool,                                     // TODO: tidy
    current_mouse_pos: winit::dpi::PhysicalPosition<f64>, // TODO: tidy
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    quad: quad::Quad,
    render_pipeline: wgpu::RenderPipeline,
    uniforms_bindings: uniforms_bindings::UniformsBindings,
    camera: camera::Camera,
    scene_bind_group: wgpu::BindGroup,
    result: result::Result,
}

impl BvhRaytracing {
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
                    features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
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

        // Constants & window uniforms bindings
        let constants = constants::Constants::new();
        let window = window::Window::new(&size);
        let uniforms_bindings =
            uniforms_bindings::UniformsBindings::new(&device, &[constants], &[window]);

        // Camera
        let camera = camera::Camera::new(
            &device,
            Point3::<f32>::new(13.0, 3.0, 3.0),
            Point3::<f32>::new(0.0, 1.0, 0.0),
            Vector3::<f32>::new(0.0, 1.0, 0.0),
            20.0,
            window,
            0.1,
            14.0,
        );

        // Scene
        let scene = scene::Scene::final_scene();
        let (scene_bind_group_layout, scene_bind_group, _) =
            buffer_bindings::create_device_buffer_binding(
                &scene.spheres[..],
                &device,
                wgpu::BufferUsages::STORAGE,
                wgpu::BufferBindingType::Storage { read_only: (true) },
            );

        // Create basic quad to render fragments onto.
        let quad = quad::Quad::new(&device);

        // Create the result texture to store current calculation status
        let result = result::Result::new(&device, &queue, window);

        // Load shader
        let shader = device.create_shader_module(&wgpu::include_wgsl!("bvh_raytracing.wgsl"));

        // Create the render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &uniforms_bindings.get_bind_group_layout(),
                    &camera.get_bind_group_layout(),
                    &scene_bind_group_layout,
                    &result.get_bind_group_layout(),
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
            mouse_down: false,
            current_mouse_pos: winit::dpi::PhysicalPosition::new(0.0, 0.0),
            surface,
            device,
            queue,
            // config,
            size,
            quad,
            render_pipeline,
            uniforms_bindings,
            camera,
            scene_bind_group,
            result,
        }
    }

    // pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    //     if new_size.width > 0 && new_size.height > 0 {
    //         self.size = new_size;
    //         self.config.width = new_size.width;
    //         self.config.height = new_size.height;

    //         self.surface.configure(&self.device, &self.config);

    //         let window = window::Window::new(&self.size);
    //         self.uniforms_bindings
    //             .update_window_buffer(&self.queue, &[window]);

    //         self.camera.set_window(window);
    //         self.camera.update(&self.queue);
    //     }
    // }

    // pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
    //     self.size
    // }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            // Movement events for arcball camera
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Pressed,
                ..
            } => {
                self.mouse_down = true;
            }
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Released,
                ..
            } => {
                self.mouse_down = false;
                self.current_mouse_pos = winit::dpi::PhysicalPosition::new(0.0, 0.0);
            }
            WindowEvent::CursorMoved { position: pos, .. } => {
                if self.mouse_down {
                    if self.current_mouse_pos.x > 0.001 && self.current_mouse_pos.y > 0.001 {
                        self.camera.rotate(
                            &self.device,
                            &self.queue,
                            &mut self.result,
                            self.size,
                            Vector2::<f32>::new(
                                self.current_mouse_pos.x as f32,
                                self.current_mouse_pos.y as f32,
                            ),
                            Vector2::<f32>::new(pos.x as f32, pos.y as f32),
                        );
                    }
                    self.current_mouse_pos = *pos;
                }
            }
            _ => {}
        }
        true
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
            render_pass.set_bind_group(0, &self.uniforms_bindings.get_bind_group(), &[]);
            render_pass.set_bind_group(1, &self.camera.get_bind_group(), &[]);
            render_pass.set_bind_group(2, &self.scene_bind_group, &[]);
            render_pass.set_bind_group(3, &self.result.get_bind_group(), &[]);
            render_pass.draw_indexed(0..self.quad.num_indices, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Update the result index (as the fragment shader has just been executed)
        self.result.increment_pass_index(&mut self.queue);
        // println!("Pass index: {}", self.result.get_pass_index());

        Ok(())
    }
}