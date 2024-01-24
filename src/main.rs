use bismuth::loader::load;
use bismuth::rendering::{camera::Camera, renderer::Renderer};

use bevy_math::prelude::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let camera = Camera {
        eye: Vec3::new(1.0, 1.0, 1.0),
        target: Vec3::ZERO,
        up: Vec3::Y,
        fovy: 75.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let (vertices, indices) = load("assets/cube.gltf").expect("Failed to load cube!");

    let mut renderer = pollster::block_on(Renderer::new(window));
    renderer.create_mesh(vertices, indices, &Mat4::IDENTITY);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == renderer.get_window().id() => {
                if !input(event) {
                    match event {
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
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize_surface(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            renderer.resize_surface(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == renderer.get_window().id() => {
                update();

                match renderer.render(&camera) {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        renderer.resize_surface(renderer.get_size())
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                renderer.get_window().request_redraw();
            }
            _ => {}
        }
    });
}

fn update() {}

fn input(event: &WindowEvent) -> bool {
    false
}
