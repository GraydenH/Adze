use glow::{HasContext};
use nalgebra_glm as glm;

use camera::OrthographicCamera;
use shader::Shader;
use texture::Texture;

use crate::glm::{Vec2, Vec3, Vec4, Mat4};
use crate::renderer::buffer::{BufferElement, BufferLayout, IndexBuffer, ShaderDataType, VertexArray, VertexBuffer};
use core::mem;
use crate::renderer::camera::PerspectiveCamera;

pub mod buffer;
pub mod camera;
pub mod shader;
pub mod texture;
pub mod renderer_2d;

pub struct Renderer {
    gl: glow::Context,
    shader: Shader,
    texture: Texture,
    pub vertex_array: VertexArray,
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Renderer {
        // Shader sources
        let vs_src: &str = "
                #version 330 core

                layout (location = 0) in vec3 aposition;
                layout (location = 1) in vec3 acolor;
                layout (location = 2) in vec2 atexture_coordinate;

                out vec3 color;
                out vec2 texture_coordinate;

                uniform mat4 model;
                uniform mat4 view;
                uniform mat4 projection;

                void main()
                {
                    gl_Position = projection * view * model * vec4(aposition, 1.0);
                    color = acolor;
                    texture_coordinate = atexture_coordinate;
                }
            ";

        let fs_src: &str = "
                #version 330 core

                // Outputs colors in RGBA
                out vec4 FragColor;

                // Inputs the color from the Vertex Shader
                in vec3 color;
                // Inputs the texture coordinates from the Vertex Shader
                in vec2 texture_coordinate;

                // Gets the Texture Unit from the main function
                uniform sampler2D tex0;

                void main()
                {
                    FragColor = texture(tex0, texture_coordinate);
                }
            ";
        let shader = Shader::new(&gl, vs_src, fs_src);
        let mut texture = Texture::new(String::from("sandbox/assets/textures/brick.png"), 1.0);
        texture.init(&gl);

        // Vertices coordinates
        let vertices: Vec<f32> = vec![
            //     COORDINATES     /        COLORS      /   TexCoord  //
            -0.5, 0.0,  0.5,     0.83, 0.70, 0.44,	0.0, 0.0,
            -0.5, 0.0, -0.5,     0.83, 0.70, 0.44,	5.0, 0.0,
            0.5, 0.0, -0.5,     0.83, 0.70, 0.44,	0.0, 0.0,
            0.5, 0.0,  0.5,     0.83, 0.70, 0.44,	5.0, 0.0,
            0.0, 0.8,  0.0,     0.92, 0.86, 0.76,	2.5, 5.0
        ];

        // Indices for vertices order
        let indices: Vec<u32> = vec![
            0, 1, 2,
            0, 2, 3,
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
            3, 0, 4
        ];
        let layout = BufferLayout::new(
            vec![
                BufferElement::new("aposition".parse().unwrap(), ShaderDataType::Float3, false),
                BufferElement::new("acolor".parse().unwrap(), ShaderDataType::Float3, false),
                BufferElement::new("atexture_coordinate".parse().unwrap(), ShaderDataType::Float2, false),
            ]
        );

        let index_buffer = IndexBuffer::new(&gl, indices);
        let vertex_buffer = VertexBuffer::new(&gl, vertices, layout);
        let vertex_array = VertexArray::new(&gl, index_buffer, vertex_buffer);

        Renderer {
            gl,
            shader,
            texture,
            vertex_array
        }
    }

    pub fn begin(&mut self, camera: &PerspectiveCamera) {
    }

    pub fn end(&mut self) {
    }

    pub fn draw(&self, rotation: f32) {
        unsafe {
            self.shader.bind(&self.gl);
            self.shader.upload_uniform_mat4(&self.gl, "model",  &glm::rotate(&glm::identity(), rotation, &glm::vec3(0.0, 1.0, 0.0)));
            self.shader.upload_uniform_mat4(&self.gl, "view",  &glm::translate(&glm::identity(), &glm::vec3(0.0, -0.5, -2.0)));
            self.shader.upload_uniform_mat4(&self.gl, "projection",  &Mat4::new_perspective((800.0 / 600.0), 45.0, 0.01, 100.0));

            Texture::bind(&self.gl, self.texture.get_renderer_id().unwrap(), 0);

            self.shader.upload_uniform_integer1(&self.gl, "tex0", 0);

            self.vertex_array.bind(&self.gl);

            self.gl.draw_elements(glow::TRIANGLES, self.vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn borrow_context(&self) -> &glow::Context {
        &self.gl
    }

    pub fn clear(&self) {
        unsafe {
            self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.clear_depth_f64(1.0);
            self.gl.depth_func(glow::LESS);
            self.gl.depth_mask(true);
            self.gl.enable(glow::DEPTH_TEST);
        }
    }
}