mod aabb;
mod buffer_bindings;
mod bvh_node;
mod camera;
mod constant_medium;
mod constants;
mod construction_scene;
mod construction_scene_bvh_node;
mod cuboid;
mod hittable_primitive;
mod interactive_section;
mod linear_constant_medium;
mod linear_hittable;
mod linear_scene_bvh;
mod material;
mod quad;
mod result;
mod scenes;
mod sphere;
mod uniforms_bindings;
mod util;
mod vertex;
mod window;

use cgmath::{Matrix4, Point3, SquareMatrix, Vector2, Vector3};
use winit::{event::WindowEvent, window::Window};

// Some bits need to be tidied into more granular structs.
pub struct BvhRaytracing {
    input_mouse_down: bool,                                     // TODO: tidy
    current_input_mouse_pos: winit::dpi::PhysicalPosition<f64>, // TODO: tidy
    rot_mouse_down: bool,                                       // TODO: tidy
    current_rot_mouse_pos: winit::dpi::PhysicalPosition<f64>,   // TODO: tidy
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    quad: quad::Quad,
    render_pipeline: wgpu::RenderPipeline,
    uniforms_bindings: uniforms_bindings::UniformsBindings,
    camera: camera::Camera,
    interactive_section: interactive_section::InteractiveSection,
    scene_bvh_bind_group: wgpu::BindGroup,
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
            Point3::<f32>::new(0.0, 0.0, 4.0),
            Point3::<f32>::new(0.0, 0.0, 0.0),
            Vector3::<f32>::new(0.0, 1.0, 0.0),
            25.0,
            window,
            0.0,
            4.0,
            0.1,
        );

        // Scene
        // let scene_bvh = scenes::stress_test_scene();
        // let scene_bvh = scenes::final_scene();
        // let scene_bvh = scenes::cornell_box();
        let mut scene_bvh = scenes::test_scene();
        // let scene_bvh = scenes::simple_scene();
        let (scene_bvh_bind_group_layout, scene_bvh_bind_group) =
            scene_bvh.create_device_buffer_binding(&device);

        // Interactive Section
        let interactive_section = interactive_section::InteractiveSection::new(
            &device,
            Matrix4::from_nonuniform_scale(2.0, 2.0, 0.1),
            scene_bvh.slice_plane_buffer.unwrap(),
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
                    &scene_bvh_bind_group_layout,
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
            input_mouse_down: false,
            current_input_mouse_pos: winit::dpi::PhysicalPosition::new(0.0, 0.0),
            rot_mouse_down: false,
            current_rot_mouse_pos: winit::dpi::PhysicalPosition::new(0.0, 0.0),
            surface,
            device,
            queue,
            size,
            quad,
            render_pipeline,
            uniforms_bindings,
            camera,
            interactive_section,
            scene_bvh_bind_group,
            result,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            // Movements events for interactive drag
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                self.input_mouse_down = true;
            }
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Released,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                self.input_mouse_down = false;
                self.current_input_mouse_pos = winit::dpi::PhysicalPosition::new(0.0, 0.0);
            }
            // Movement events for arcball camera
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Right,
                ..
            } => {
                self.rot_mouse_down = true;
            }
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Released,
                button: winit::event::MouseButton::Right,
                ..
            } => {
                self.rot_mouse_down = false;
                self.current_rot_mouse_pos = winit::dpi::PhysicalPosition::new(0.0, 0.0);
            }

            WindowEvent::CursorMoved { position: pos, .. } => {
                // If currently rotating
                if self.rot_mouse_down {
                    if self.current_rot_mouse_pos.x > 0.001 && self.current_rot_mouse_pos.y > 0.001
                    {
                        self.camera.rotate(
                            &self.device,
                            &self.queue,
                            &mut self.result,
                            self.size,
                            Vector2::<f32>::new(
                                self.current_rot_mouse_pos.x as f32,
                                self.current_rot_mouse_pos.y as f32,
                            ),
                            Vector2::<f32>::new(pos.x as f32, pos.y as f32),
                        );
                    }
                    self.current_rot_mouse_pos = *pos;
                } else if self.input_mouse_down {
                    // Else if we are dragging an input command (e.g. moving the interactive section)
                    if self.current_input_mouse_pos.x > 0.001
                        && self.current_input_mouse_pos.y > 0.001
                    {
                        self.interactive_section.translate(
                            &self.device,
                            &self.queue,
                            &mut self.result,
                            self.size,
                            Vector2::<f32>::new(
                                self.current_input_mouse_pos.x as f32,
                                self.current_input_mouse_pos.y as f32,
                            ),
                            Vector2::<f32>::new(pos.x as f32, pos.y as f32),
                        );
                    }
                    self.current_input_mouse_pos = *pos;
                }
            }
            WindowEvent::MouseWheel {
                delta: winit::event::MouseScrollDelta::LineDelta(_, pos_y),
                ..
            } => {
                self.camera.zoom(
                    &self.device,
                    &self.queue,
                    &mut self.result,
                    self.size,
                    *pos_y,
                );
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
            render_pass.set_bind_group(2, &self.scene_bvh_bind_group, &[]);
            render_pass.set_bind_group(3, &self.result.get_bind_group(), &[]); // We are limited to 4 bind groups
            render_pass.draw_indexed(0..self.quad.num_indices, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Update the result index (as the fragment shader has just been executed)
        self.result.increment_pass_index(&mut self.queue);

        Ok(())
    }
}
