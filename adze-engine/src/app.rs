use std::str;
use glow::HasContext;
use crate::buffer::{VertexBuffer, IndexBuffer, VertexArray, BufferLayout, BufferElement, ShaderDataType};
use crate::shader::Shader;
use crate::renderer::Renderer;
use crate::layer::{LayerStack, Layer};

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

    pub fn run(self) {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();

        let (gl_window, gl) = create_display(&event_loop, self.title.as_str());

        let egui = egui_glow::EguiGlow::new(&gl_window, &gl);

        let renderer = Renderer::new(gl);
        let mut layer_stack = self.layer_stack;

        event_loop.run(move |event, _, control_flow| {
            use glutin::event::{Event, WindowEvent};
            use glutin::event_loop::ControlFlow;
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        // Cleanup
                        unsafe {
                            // gl::DeleteProgram(program);
                            // gl::DeleteShader(fs);
                            // gl::DeleteShader(vs);
                            // gl::DeleteBuffers(1, &vbo);
                            // gl::DeleteVertexArrays(1, &vao);
                        }
                        *control_flow = ControlFlow::Exit
                    },
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    for layer in layer_stack.iter_mut().rev() {
                        layer.on_tick(&renderer);
                    }

                    gl_window.swap_buffers().unwrap();
                },
                _ => (),
            }
        });
    }
}