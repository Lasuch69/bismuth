pub mod loader;
pub mod rendering;

use loader::load;
use rendering::{camera::Camera, renderer::Renderer};

use bevy_ecs::{schedule::Schedule, world::World};
use bevy_math::prelude::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct App {
    world: World,
    renderer: Renderer,
    event_loop: EventLoop<()>,

    update_schedule: Schedule,
}

impl App {
    pub fn new() -> Self {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let (vertices, indices) = load("assets/cube.gltf").expect("Failed to load cube!");

        let mut renderer = pollster::block_on(Renderer::new(window));
        renderer.create_mesh(vertices, indices, &Mat4::IDENTITY);

        let camera = Camera {
            eye: Vec3::new(1.0, 1.0, 1.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fovy: 75.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut world = World::new();
        world.insert_resource(camera);

        let update_schedule = Schedule::default();

        Self {
            world,
            renderer,
            event_loop,
            update_schedule,
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.renderer.get_window().id() => {
                    // handle input
                    //if input(event) { return; }

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
                            self.renderer.resize_surface(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            self.renderer.resize_surface(**new_inner_size);
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id)
                    if window_id == self.renderer.get_window().id() =>
                {
                    self.update_schedule.run(&mut self.world);
                    let camera = self.world.get_resource::<Camera>();

                    match self.renderer.render(camera) {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            self.renderer.resize_surface(self.renderer.get_size())
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.renderer.get_window().request_redraw();
                }
                _ => {}
            }
        });
    }
}
