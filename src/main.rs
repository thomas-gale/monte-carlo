mod basic_raytracing;

use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

async fn run() {
    env_logger::init();

    // Create the window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("monte carlo")
        .with_resizable(false)
        .with_inner_size(PhysicalSize::new(512, 512))
        // .set_min_dimensions(256, 256)
        // .set_max_dimensions(256, 256)
        // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();

    // Create the renderers
    let mut basic_renderer = basic_raytracing::BasicRaytracing::new(&window).await;

    // println!("Press 'return' to render the scene to the window!");
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            // WindowEvent::Resized(physical_size) => {
            //     basic_renderer.resize(*physical_size);
            // }
            // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            //     // new_inner_size is &&mut so we have to dereference it twice
            //     basic_renderer.resize(**new_inner_size);
            // }
            _ => {
                // Process input events
                basic_renderer.input(event);
            }
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            match basic_renderer.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                // Err(wgpu::SurfaceError::Lost) => basic_renderer.resize(basic_renderer.get_size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e), }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

fn main() {
    pollster::block_on(run());
}
