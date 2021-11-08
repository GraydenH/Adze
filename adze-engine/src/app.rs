use std::str;
use glow::HasContext;
use crate::buffer::{VertexBuffer, IndexBuffer, VertexArray, BufferLayout, BufferElement, ShaderDataType};
use crate::shader::Shader;
use crate::renderer::Renderer;

// Shader sources
static VS_SRC: &'static str = "
        #version 330 core

        layout(location = 0) in vec3 a_position;
        layout(location = 1) in vec4 a_color;

        out vec3 v_position;
        out vec4 v_color;

        void main()
        {
            v_position = a_position;
            v_color = a_color;
            gl_Position = vec4(a_position, 1.0);
        }
";

const FS_SRC: &'static str = "
        #version 330 core

        layout(location = 0) out vec4 color;

        in vec3 v_position;
        in vec4 v_color;

        void main()
        {
            color = vec4(v_position * 0.5 + 0.5, 1.0);
            color = v_color;
        }
";

const SQUARE_VS_SRC: &str = "
        #version 330 core

        layout(location = 0) in vec3 a_position;

        out vec3 v_Position;

        void main()
        {
            v_Position = a_position;
            gl_Position = vec4(a_position, 1.0);
        }
";

const SQUARE_FS_SRC: &str = "
        #version 330 core

        layout(location = 0) out vec4 color;

        in vec3 v_Position;

        void main()
        {
            color = vec4(0.2, 0.3, 0.8, 1.0);
        }
";

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
    title: String
}

impl App {
    pub fn new(title: &str) -> App {
        App {
            title: String::from(title)
        }
    }

    pub fn run(&self) {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();

        let (gl_window, gl) = create_display(&event_loop, self.title.as_str());

        let egui = egui_glow::EguiGlow::new(&gl_window, &gl);

        let renderer = Renderer::new();

        let shader = Shader::new(&gl, VS_SRC, FS_SRC);
        let square_shader = Shader::new(&gl, SQUARE_VS_SRC, SQUARE_FS_SRC);

        let vertices = vec![
            -0.5, -0.5, 0.0, 0.8, 0.2, 0.8, 1.0,
            0.5, -0.5, 0.0, 0.2, 0.3, 0.8, 1.0,
            0.0, 0.5, 0.0, 0.8, 0.8, 0.2, 1.0
        ];

        let indices = vec![
            0, 1, 2
        ];


        let layout = BufferLayout::new(
            vec![
                BufferElement::new("a_position".parse().unwrap(), ShaderDataType::Float3, false),
                BufferElement::new("a_color".parse().unwrap(), ShaderDataType::Float4, false),
            ]
        );

        let vertex_buffer = VertexBuffer::new(&gl, vertices, layout);
        let index_buffer = IndexBuffer::new(&gl, indices);

        let mut vertex_array = VertexArray::new(&gl, index_buffer);
        vertex_array.add_vertex_buffer(&gl, vertex_buffer);

        let square_vertices = vec![
            -0.75, -0.75, 0.0,
            0.75, -0.75, 0.0,
            0.75, 0.75, 0.0,
            -0.75, 0.75, 0.0
        ];

        let square_layout = BufferLayout::new(
            vec![
                BufferElement::new("a_position".parse().unwrap(), ShaderDataType::Float3, false)
            ]
        );

        let square_vertex_buffer = VertexBuffer::new(&gl, square_vertices, square_layout);

        let square_indices = vec![0, 1, 2, 2, 3, 0];
        let square_index_buffer = IndexBuffer::new(&gl, square_indices);

        let mut square_vertex_array = VertexArray::new(&gl, square_index_buffer);
        square_vertex_array.add_vertex_buffer(&gl, square_vertex_buffer);

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
                    renderer.clear(&gl);
                    renderer.begin();

                    square_shader.bind(&gl);
                    renderer.draw(&gl, &square_vertex_array);

                    shader.bind(&gl);
                    renderer.draw(&gl, &vertex_array);

                    renderer.end();

                    gl_window.swap_buffers().unwrap();
                },
                _ => (),
            }
        });
    }
}