use std::str;
use std::time::Instant;

use glow::HasContext;
use glutin::dpi::PhysicalPosition;
use glutin::event::{DeviceEvent, VirtualKeyCode};

use crate::glm;
use crate::glm::Vec2;
use crate::glutin::dpi::LogicalPosition;
use crate::glutin::event::ElementState;
use crate::glutin::event::MouseScrollDelta::{LineDelta, PixelDelta};
use crate::renderer::buffer::{BufferElement, BufferLayout, IndexBuffer, ShaderDataType, VertexArray, VertexBuffer};
use crate::renderer::Renderer;
use crate::shader::Shader;
use crate::app::layer::{LayerStack, Layer};

pub mod event;
pub mod layer;

pub static mut KEY_PRESSED: [bool; 149] = [false; 149];

fn create_display(
    event_loop: &glutin::event_loop::EventLoop<()>,
    title: &str
) -> (
    glutin::WindowedContext<glutin::PossiblyCurrent>,
    glow::Context,
) {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title(title);

    let gl_window = unsafe {
        glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    let gl = unsafe { glow::Context::from_loader_function(|s| gl_window.get_proc_address(s)) };

    unsafe {
        use glow::HasContext as _;
        gl.enable(glow::FRAMEBUFFER_SRGB);
    }

    (gl_window, gl)
}

pub struct App {
    title: String,
    layer_stack: LayerStack
}

impl App {
    pub fn new(title: &str) -> App {
        App {
            title: String::from(title),
            layer_stack: LayerStack::new()
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layer_stack.push_layer(layer);
    }

    pub fn is_key_pressed(key_code: VirtualKeyCode) -> bool {
        unsafe { return KEY_PRESSED [key_code as usize]; }
    }

    pub fn run(mut self) {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();

        let (gl_window, gl) = create_display(&event_loop, self.title.as_str());

        let mut egui = egui_glow::EguiGlow::new(&gl_window, &gl);

        let mut renderer = Renderer::new(gl);
        let mut layer_stack = self.layer_stack;

        let clock = Instant::now();
        let fixed_timestep = 1.0 / 60.0;
        let mut elapsed_time = 0.0;
        let mut dt = clock.elapsed().as_secs_f32() - elapsed_time;

        event_loop.run(move |event, _, control_flow| {
            use glutin::event::{Event, WindowEvent};
            use glutin::event_loop::ControlFlow;

            *control_flow = ControlFlow::Wait;

            let mut redraw = || {
                egui.begin_frame(gl_window.window());

                for layer in layer_stack.iter_mut().rev() {
                    layer.on_ui_update(&egui);
                }

                let (needs_repaint, shapes) = egui.end_frame(gl_window.window());

                {
                    // draw things behind egui here
                    dt = clock.elapsed().as_secs_f32() - elapsed_time;
                    while dt >= fixed_timestep {
                        dt -= fixed_timestep;
                        elapsed_time += fixed_timestep;

                        for layer in layer_stack.iter_mut().rev() {
                            layer.on_tick(&mut renderer);
                        }
                    }
                    egui.paint(&gl_window, renderer.borrow_context(), shapes);

                    // draw things on top of egui here

                    gl_window.swap_buffers().unwrap();
                }
            };

            match event {
                Event::RedrawEventsCleared if cfg!(windows) => redraw(),
                Event::RedrawRequested(_) if !cfg!(windows) => redraw(),
                Event::MainEventsCleared => {
                    gl_window.window().request_redraw();
                },
                Event::DeviceEvent { event, ..} => match event {
                    DeviceEvent::Key(input) => unsafe {
                        if let Some(keycode) = input.virtual_keycode {
                            let index = keycode as u16;

                            if input.state == ElementState::Pressed {
                                let repeat = KEY_PRESSED[index as usize];
                                KEY_PRESSED[index as usize] = true;
                                for layer in layer_stack.iter_mut().rev() {
                                    layer.on_key_press(keycode, repeat);
                                }
                            } else {
                                KEY_PRESSED[index as usize] = false;
                                for layer in layer_stack.iter_mut().rev() {
                                    layer.on_key_release(keycode);
                                }
                            };
                        }
                    }
                    _ => {}
                },
                Event::WindowEvent { event, .. } => {
                    if egui.is_quit_event(&event) {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }

                    if let glutin::event::WindowEvent::Resized(physical_size) = event {
                        gl_window.resize(physical_size);
                    }

                    egui.on_event(&event);

                    gl_window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
                }
                Event::RedrawRequested(_) => {
                    //gl_window.swap_buffers().unwrap();
                },
                Event::LoopDestroyed => {
                    egui.destroy(&renderer.borrow_context());
                }
                _ => (),
            }
        });
    }
}
