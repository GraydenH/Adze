use crate::buffer::{VertexArray, BufferLayout, BufferElement, VertexBuffer, IndexBuffer, ShaderDataType};
use core::{ptr, mem};
use glow::{HasContext, PixelUnpackData};
use crate::buffer;
use crate::shader::Shader;
use crate::camera::OrthographicCamera;
use nalgebra_glm::Mat4;
use nalgebra_glm as glm;
use crate::glm::{Vec4, Vec3, Vec2};
use image::{GenericImageView, DynamicImage};
use crate::texture::Texture;

// Shader sources
const TEXTURE_VS_SRC: &str = "
        #version 330 core

        layout(location = 0) in vec3 aposition;
        layout(location = 1) in vec4 acolor;
        layout(location = 2) in vec2 atexture_coordinate;
        layout(location = 3) in float atexture_index;

        uniform mat4 uprojection_view;

        out vec2 vtexture_coordinate;
        out vec4 vcolor;
        out float vtexture_index;

        void main() {
            vtexture_coordinate = atexture_coordinate;
            vtexture_index = atexture_index;
            vcolor = acolor;
            gl_Position = uprojection_view * vec4(aposition, 1.0);
        }
";

const TEXTURE_FS_SRC: &str = "
        #version 330 core
        layout(location = 0) out vec4 color;

        in vec4 vcolor;
        in vec2 vtexture_coordinate;
        in float vtexture_index;

        uniform sampler2D utextures[32];

        void main() {
            color = texture(utextures[int(vtexture_index)], vtexture_coordinate) * vcolor;
        }
";

const MAX_QUADS: usize = 10000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;
const MAX_TEXTURE_SLOTS: usize = 32;

#[derive(Clone, Copy, Debug)]
pub struct QuadVertex {
    position: Vec3,
    color: Vec4,
    texture_coordinate: Vec2,
    texture_index: f32
}

pub struct Renderer {
    gl: glow::Context,
    vertex_array: VertexArray,
    shader: Shader,
    white_texture: Texture,
    quad_vertices: Vec<QuadVertex>,
    index_count: i32,
    textures: Vec<glow::Texture>
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Renderer {
        let shader = Shader::new(&gl, TEXTURE_VS_SRC, TEXTURE_FS_SRC);
        shader.bind(&gl);

        let layout = BufferLayout::new(
            vec![
                BufferElement::new("aposition".parse().unwrap(), ShaderDataType::Float3, false),
                BufferElement::new("acolor".parse().unwrap(), ShaderDataType::Float4, false),
                BufferElement::new("atexture_coordinate".parse().unwrap(), ShaderDataType::Float2, false),
                BufferElement::new("atexture_index".parse().unwrap(), ShaderDataType::Float1, false),
            ]
        );

        let mut offset = 0;
        let mut indices = Vec::new();
        for _ in (0..MAX_INDICES).step_by(6) {
            indices.push(offset);
            indices.push(offset + 1);
            indices.push(offset + 2);

            indices.push(offset + 2);
            indices.push(offset + 3);
            indices.push(offset);

            offset += 4;
        }

        let index_buffer = IndexBuffer::new(&gl, indices);
        let vertex_buffer = VertexBuffer::from_size(&gl, ((MAX_VERTICES * mem::size_of::<QuadVertex>()) as i32), layout);
        let vertex_array = VertexArray::new(&gl, index_buffer, vertex_buffer);

        let white_texture = Texture::from_data(&gl, vec![255_u8, 255_u8, 255_u8, 255_u8], 1, 1, glow::RGBA8, glow::RGBA);

        // let mut white_texture = Texture::from_dimensions(&gl,1, 1);
        // white_texture.set_data(&gl, vec![255_u8, 255_u8, 255_u8, 255_u8]);
        let white_texture_renderer_id = white_texture.get_renderer_id().unwrap();

        let mut samplers: Vec<i32> = Vec::new();
        for i in 0..MAX_TEXTURE_SLOTS {
            samplers.push(i as i32);
        }

        shader.upload_uniform_int_array(&gl, "utextures", samplers);

        Renderer::init(&gl);

        Renderer {
            gl,
            vertex_array,
            shader,
            white_texture,
            quad_vertices: vec![],
            index_count: 0,
            textures: vec![white_texture_renderer_id]
        }
    }

    fn init(gl: &glow::Context) {
        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            gl.enable(glow::DEPTH_TEST);
        }
    }

    pub fn begin(&mut self, camera: &OrthographicCamera) {
        self.shader.bind(&self.gl);
        self.shader.upload_uniform_mat4(&self.gl, "uprojection_view",  &camera.get_projection_view());

        self.reset();
    }

    pub fn end(&mut self) {
        self.vertex_array.set_vertices(&self.gl, &self.quad_vertices);
        self.flush();
    }

    fn reset(&mut self) {
        for i in 1..self.textures.len() {
            self.textures.pop();
        }
        self.quad_vertices = Vec::new();
        self.index_count = 0;
    }

    fn flush(&mut self) {
        self.vertex_array.set_vertices(&self.gl, &self.quad_vertices);

        for (i, texture) in self.textures.iter().enumerate() {
            Texture::bind(&self.gl, *texture, i as u32);
        }

        self.draw();
        self.reset();
    }

    fn draw(&self) {
        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn clear(&self) {
        // Clear the screen to black
        unsafe {
            self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn borrow_context(&self) -> &glow::Context {
        &self.gl
    }

    pub fn set_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.gl.viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }

    pub fn draw_flat_color_quad(&mut self, position: Vec3, size: Vec3, color: Vec4) {
        if self.quad_vertices.len() + 4 > MAX_VERTICES as usize {
            self.flush();
        }

        self.quad_vertices.push( QuadVertex {
            position,
            color,
            texture_coordinate: glm::vec2(0.0, 0.0),
            texture_index: 0.0
        });

        self.quad_vertices.push(QuadVertex {
            position: glm::vec3(position.x + size.x, position.y, position.z),
            color,
            texture_coordinate: glm::vec2(1.0, 0.0),
            texture_index: 0.0
        });

        self.quad_vertices.push(QuadVertex {
            position: glm::vec3(position.x + size.x, position.y + size.y, position.z),
            color,
            texture_coordinate: glm::vec2(1.0, 1.0),
            texture_index: 0.0
        });

        self.quad_vertices.push(QuadVertex {
            position: glm::vec3(position.x, position.y + size.y, position.z),
            color,
            texture_coordinate: glm::vec2(0.0, 1.0),
            texture_index: 0.0
        });

        self.index_count += 6;
    }

    pub fn draw_quad(&mut self, position: Vec3, size: Vec2, texture: &mut Texture) {
        if texture.get_renderer_id() == None {
            texture.init(&self.gl);
        }

        if self.quad_vertices.len() + 4 > MAX_VERTICES as usize {
            self.flush();
        }

        let mut texture_index = 0.0;
        for (i, t) in self.textures.iter().enumerate() {
            if texture.get_renderer_id().unwrap() == *t {
                texture_index = i as f32;
            }
        }

        if texture_index == 0.0 {
            self.textures.push(texture.get_renderer_id().unwrap());
            texture_index = self.textures.len() as f32 - 1.0;
        }
        let color = glm::vec4(1.0, 1.0, 1.0,   1.0);

        self.quad_vertices.push( QuadVertex {
            position,
            color,
            texture_coordinate: glm::vec2(0.0, 0.0),
            texture_index
        });

        self.quad_vertices.push(QuadVertex {
            position: glm::vec3(position.x + size.x, position.y, position.z),
            color,
            texture_coordinate: glm::vec2(1.0, 0.0),
            texture_index
        });

        self.quad_vertices.push(QuadVertex {
            position: glm::vec3(position.x + size.x, position.y + size.y, position.z),
            color,
            texture_coordinate: glm::vec2(1.0, 1.0),
            texture_index
        });

        self.quad_vertices.push(QuadVertex {
            position: glm::vec3(position.x, position.y + size.y, position.z),
            color,
            texture_coordinate: glm::vec2(0.0, 1.0),
            texture_index
        });

        self.index_count += 6;
    }
}
