use crate::buffer::{VertexArray, BufferLayout, BufferElement, VertexBuffer, IndexBuffer, ShaderDataType};
use core::ptr;
use glow::HasContext;
use crate::buffer;
use crate::shader::Shader;
use crate::camera::OrthographicCamera;
use nalgebra_glm::Mat4;
use nalgebra_glm as glm;

// Shader sources
static VS_SRC: &'static str = "
        #version 330 core

        layout(location = 0) in vec3 a_position;
        layout(location = 1) in vec4 a_color;

        out vec3 v_position;
        out vec4 v_color;

        uniform mat4 uprojection_view;

        void main()
        {
            v_position = a_position;
            v_color = a_color;
            gl_Position = uprojection_view * vec4(a_position, 1.0);
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

        uniform mat4 uprojection_view;

        void main()
        {
            v_Position = a_position;
            gl_Position = uprojection_view * vec4(a_position, 1.0);
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

pub struct Renderer {
    gl: glow::Context,
    vertex_array: VertexArray,
    shader: Shader,
    square_vertex_array: VertexArray,
    square_shader: Shader,
    projection_view: Mat4
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Renderer {
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
        Renderer {
            gl,
            vertex_array,
            shader,
            square_vertex_array,
            square_shader,
            projection_view: glm::identity()
        }
    }

    pub fn begin(&mut self, camera: &OrthographicCamera) {
        self.projection_view = camera.get_projection_view();
    }

    pub fn end(&self) {

    }

    pub fn clear(&self) {
        // Clear the screen to black
        unsafe {
            self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_triangle(&self) {
        self.vertex_array.bind(&self.gl);
        self.shader.bind(&self.gl);
        self.shader.upload_uniform_mat4(&self.gl, "uprojection_view", &self.projection_view);
        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, self.vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn draw_square(&self) {
        self.square_vertex_array.bind(&self.gl);
        self.square_shader.bind(&self.gl);
        self.square_shader.upload_uniform_mat4(&self.gl, "uprojection_view", &self.projection_view);
        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, self.square_vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }
}