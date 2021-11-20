use crate::buffer::{VertexArray, BufferLayout, BufferElement, VertexBuffer, IndexBuffer, ShaderDataType};
use core::ptr;
use glow::{HasContext, PixelUnpackData};
use crate::buffer;
use crate::shader::Shader;
use crate::camera::OrthographicCamera;
use nalgebra_glm::Mat4;
use nalgebra_glm as glm;
use crate::glm::{Vec4, Vec3};
use image::{GenericImageView, DynamicImage};
use crate::texture::Texture;

// Shader sources
const FLAT_COLOR_VS_SRC: &str = "
        #version 330 core

        layout(location = 0) in vec3 aposition;

        out vec3 vposition;

        uniform mat4 uprojection_view;
        uniform mat4 utransform;

        void main()
        {
            vposition = aposition;
            gl_Position = uprojection_view * utransform * vec4(aposition, 1.0);
        }
";

const FLAT_COLOR_FS_SRC: &str = "
        #version 330 core

        layout(location = 0) out vec4 color;

        in vec3 vposition;

        uniform vec3 ucolor;

        void main()
        {
            color = vec4(ucolor, 1.0);
        }
";

const TEXTURE_VS_SRC: &str = "
        #version 330 core

        layout(location = 0) in vec3 aposition;
        layout(location = 1) in vec2 atex_coord;

        uniform mat4 uprojection_view;
        uniform mat4 utransform;

        out vec2 vtex_coord;

        void main() {
            vtex_coord = atex_coord;
            gl_Position = uprojection_view * utransform * vec4(aposition, 1.0);
        }

";

const TEXTURE_FS_SRC: &str = "
        #version 330 core

        layout(location = 0) out vec4 color;

        in vec2 vtex_coord;

        uniform sampler2D utexture;
        uniform float utiling;

        void main() {
            color = texture(utexture, vtex_coord * utiling);
        }
";

pub struct Renderer {
    gl: glow::Context,
    quad_vertex_array: VertexArray,
    flat_color_shader: Shader,
    texture_shader: Shader,
    projection_view: Mat4
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Renderer {
        let flat_color_shader = Shader::new(&gl, FLAT_COLOR_VS_SRC, FLAT_COLOR_FS_SRC);
        let texture_shader = Shader::new(&gl, TEXTURE_VS_SRC, TEXTURE_FS_SRC);

        let vertices = vec![
            -0.5, -0.5, 0.0, 0.0, 0.0,
            0.5, -0.5, 0.0, 1.0, 0.0,
            0.5,  0.5, 0.0, 1.0, 1.0,
            -0.5,  0.5, 0.0, 0.0, 1.0,
        ];

        let ayout = BufferLayout::new(
            vec![
                BufferElement::new("aposition".parse().unwrap(), ShaderDataType::Float3, false),
                BufferElement::new("atexture_coord".parse().unwrap(), ShaderDataType::Float2, false),
            ]
        );

        let vertex_buffer = VertexBuffer::new(&gl, vertices, ayout);

        let indices = vec![0, 1, 2, 2, 3, 0];
        let square_index_buffer = IndexBuffer::new(&gl, indices);

        let mut quad_vertex_array = VertexArray::new(&gl, square_index_buffer);
        quad_vertex_array.add_vertex_buffer(&gl, vertex_buffer);

        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            gl.enable(glow::DEPTH_TEST);
        }

        Renderer {
            gl,
            quad_vertex_array,
            flat_color_shader,
            texture_shader,
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
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn set_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.gl.viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }

    pub fn draw_flat_color_quad(&self, position: Vec3, size: Vec3, color: Vec3) {
        self.quad_vertex_array.bind(&self.gl);
        self.flat_color_shader.bind(&self.gl);
        self.flat_color_shader.upload_uniform_float3(&self.gl, "ucolor", color);
        self.flat_color_shader.upload_uniform_mat4(&self.gl, "uprojection_view", &self.projection_view);

        let transform = glm::translate(&glm::identity(), &position) *
            //glm::rotate(&glm::identity(),rotation,&s) *
            glm::scale(&glm::identity(), &size);

        self.flat_color_shader.upload_uniform_mat4(&self.gl, "utransform", &transform);

        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, self.quad_vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn draw_quad_with_texture(&self, position: Vec3, size: Vec3, texture: &mut Texture) {
        self.quad_vertex_array.bind(&self.gl);
        self.texture_shader.bind(&self.gl);
        self.texture_shader.upload_uniform_mat4(&self.gl, "uprojection_view", &self.projection_view);

        let transform = glm::translate(&glm::identity(), &position) *
            //glm::rotate(&glm::identity(),rotation,&s) *
            glm::scale(&glm::identity(), &size);

        self.texture_shader.upload_uniform_mat4(&self.gl, "utransform", &transform);
        self.texture_shader.upload_uniform_integer1(&self.gl, "utexture", 0);
        self.texture_shader.upload_uniform_float1(&self.gl, "utiling", texture.get_tiling());

        if texture.get_renderer_id() == None {
            texture.init(&self.gl);
        }

        texture.bind(&self.gl, 0);

        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, self.quad_vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }
}