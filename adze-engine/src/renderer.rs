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

        void main() {
            color = texture(utexture, vtex_coord);
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
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_flat_color_quad(&self, transform: &Mat4, color: &Vec3) {
        self.quad_vertex_array.bind(&self.gl);
        self.flat_color_shader.bind(&self.gl);
        self.flat_color_shader.upload_uniform_float3(&self.gl, "ucolor", color);
        self.flat_color_shader.upload_uniform_mat4(&self.gl, "uprojection_view", &self.projection_view);
        self.flat_color_shader.upload_uniform_mat4(&self.gl, "utransform", transform);
        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, self.quad_vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn draw_quad_with_texture(&self, transform: &Mat4, texture: &mut Texture) {
        self.quad_vertex_array.bind(&self.gl);
        self.texture_shader.bind(&self.gl);
        self.texture_shader.upload_uniform_mat4(&self.gl, "uprojection_view", &self.projection_view);
        self.texture_shader.upload_uniform_mat4(&self.gl, "utransform", transform);
        self.texture_shader.upload_uniform_integer1(&self.gl, "utexture", 0);

        if texture.get_renderer_id() == None {
            self.init_texture(texture);
        }

        self.bind_texture(texture, 0);

        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, self.quad_vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }

    // https://www.reddit.com/r/rust/comments/7me7zr/using_image_crate_to_load_an_image_and_use_it_as/
    fn init_texture(&self, texture: &mut Texture) {
        match image::open(String::from(texture.get_path())) {
            Err(err) => panic!("Could not load image {}: {}", texture.get_path(), err),
            Ok(img) => unsafe {
                let (width, height) = img.dimensions();

                let image = match img {
                    DynamicImage::ImageRgb8(img) => img,
                    img => img.to_rgb8()
                };

                let img_data = image.into_raw();

                let renderer_id = self.gl.create_texture().unwrap();
                self.gl.bind_texture(glow::TEXTURE_2D, Some(renderer_id));
                self.gl.tex_storage_2d(glow::TEXTURE_2D, 1, glow::RGB8, width as i32, height as i32);

                self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
                self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
                self.gl.tex_sub_image_2d(glow::TEXTURE_2D,0, 0, 0, width as i32, height as i32, glow::RGB, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(img_data.as_slice()));

                texture.set_renderer_id(renderer_id);
            }
        }
    }

    fn bind_texture(&self, texture: &Texture, slot: u32) {
        unsafe {
            self.gl.active_texture(slot);
            self.gl.bind_texture(glow::TEXTURE_2D, texture.get_renderer_id());
        }
    }

    fn unbind_texture(&self, slot: u32) {
        unsafe {
            self.gl.active_texture(slot);
            self.gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}