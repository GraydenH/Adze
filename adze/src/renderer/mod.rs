use glow::{HasContext};
use nalgebra_glm as glm;

use camera::OrthographicCamera;
use shader::Shader;
use texture::Texture;

use crate::glm::{Vec2, Vec3, Vec4, Mat4};
use crate::renderer::buffer::{BufferElement, BufferLayout, IndexBuffer, ShaderDataType, VertexArray, VertexBuffer};
use core::mem;
use crate::renderer::camera::PerspectiveCamera;
use crate::renderer::mesh::Mesh;
use crate::renderer::texture::TextureType;

pub mod buffer;
pub mod camera;
pub mod shader;
pub mod texture;
pub mod renderer_2d;
pub mod mesh;

pub struct Vertex {
    position: Vec3,
    color: Vec3,
    tex_uv: Vec2,
    normal: Vec3,
}

pub struct Renderer {
    gl: glow::Context,
    shader: Shader,
    vertex_array: VertexArray,
    light_color: Vec4,
    light_position: Vec3,
    mesh: Mesh
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Renderer {
        let vs_src: &str = "
                #version 330 core

                layout (location = 0) in vec3 aposition;
                layout (location = 1) in vec3 acolor;
                layout (location = 2) in vec2 atexture_coordinate;
                layout (location = 3) in vec3 anormal;

                out vec3 color;
                out vec2 texture_coordinate;
                out vec3 normal;
                out vec3 current_position;

                uniform mat4 projection_view;
                uniform mat4 model;

                void main()
                {
                    current_position = vec3(model * vec4(aposition, 1.0f));
                    gl_Position = projection_view * vec4(current_position, 1.0);
                    color = acolor;
                    texture_coordinate = atexture_coordinate;
                    normal = anormal;
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
                in vec3 normal;
                in vec3 current_position;

                // Gets the Texture Unit from the main function
                uniform sampler2D tex0;
                uniform sampler2D tex1;
                uniform vec4 light_color;
                uniform vec3 light_position;
                uniform vec3 camera_position;

                vec4 pointLight() {
                    vec3 light = light_position - current_position;
                    float distance = length(light);
                    float a = 3.0;
                    float b = 0.7;
                    float intensity = 1.0 / (a * distance * distance + b * distance + 1.0f);

                    float ambient = 0.20f;

                    vec3 n = normalize(normal);
                    vec3 light_direction = normalize(light);
                    float diffuse = max(dot(n, light_direction), 0.0f);

                    float specular_light = 0.50f;
                    vec3 view_direction = normalize(camera_position - current_position);
                    vec3 reflection_direction = reflect(-light_direction, normal);
                    float specular_factor = pow(max(dot(view_direction, reflection_direction), 0.0f), 8);
                    float specular = specular_factor * specular_light;

                    float sum = diffuse * intensity + ambient;

                    return light_color * (texture(tex0, texture_coordinate) * vec4(sum, sum, sum, 1.0f) + texture(tex1, texture_coordinate).r * specular * intensity);
                }

                vec4 directionalLight() {
                    float ambient = 0.20f;

                    vec3 n = normalize(normal);
                    // make uniform
                    vec3 direction = vec3(1.0f, 1.0f, 0.0f);
                    vec3 light_direction = normalize(direction);
                    float diffuse = max(dot(n, light_direction), 0.0f);

                    float specular_light = 0.50f;
                    vec3 view_direction = normalize(camera_position - current_position);
                    vec3 reflection_direction = reflect(-light_direction, normal);
                    float specular_factor = pow(max(dot(view_direction, reflection_direction), 0.0f), 8);
                    float specular = specular_factor * specular_light;

                    float sum = diffuse + ambient;

                    return light_color * (texture(tex0, texture_coordinate) * vec4(sum, sum, sum, 1.0f) + texture(tex1, texture_coordinate).r * specular);
                }

                vec4 spotLight() {
                    float outer_cone = 0.90f;
                    float inner_cone = 0.95f;

                    float ambient = 0.20f;

                    vec3 n = normalize(normal);
                    vec3 light_direction = normalize(light_position - current_position);
                    float diffuse = max(dot(n, light_direction), 0.0f);

                    float specular_light = 0.50f;
                    vec3 view_direction = normalize(camera_position - current_position);
                    vec3 reflection_direction = reflect(-light_direction, normal);
                    float specular_factor = pow(max(dot(view_direction, reflection_direction), 0.0f), 8);
                    float specular = specular_factor * specular_light;

                    float angle = dot(vec3(0.0f, -1.0f, 0.0f), -light_direction);
                    float intensity = clamp((angle - outer_cone) / (inner_cone - outer_cone), 0.0f, 1.0f);

                    float sum = diffuse * intensity + ambient;

                    return light_color * (texture(tex0, texture_coordinate) * vec4(sum, sum, sum, 1.0f) + texture(tex1, texture_coordinate).r * specular * intensity);
                }

                void main() {
                    FragColor = spotLight();
                }
            ";
        let shader = Shader::new(&gl, vs_src, fs_src);

        let texture = Texture::new(&gl, String::from("adze/assets/textures/planks.png"), 1.0, TextureType::Diffuse);
        let specular_map = Texture::new(&gl, String::from("adze/assets/textures/planksSpec.png"), 1.0, TextureType::Specular);

        // Vertices coordinates
        let vertices: Vec<Vertex> = vec![
            //     COORDINATES     /        COLORS        /    TexCoord    /       NORMALS     //
            Vertex {position: glm::vec3(-1.0, 0.0, 1.0), color: glm::vec3(0.0, 0.0, 0.0), tex_uv: glm::vec2(0.0, 0.0), normal: glm::vec3(0.0, 1.0, 0.0)},
            Vertex {position: glm::vec3(-1.0, 0.0, -1.0), color: glm::vec3(0.0, 0.0, 0.0), tex_uv: glm::vec2(0.0, 1.0), normal: glm::vec3(0.0, 1.0, 0.0)},
            Vertex {position: glm::vec3(1.0, 0.0, -1.0), color: glm::vec3(0.0, 0.0, 0.0), tex_uv: glm::vec2(1.0, 1.0), normal: glm::vec3(0.0, 1.0, 0.0)},
            Vertex {position: glm::vec3(1.0, 0.0, 1.0), color: glm::vec3(0.0, 0.0, 0.0), tex_uv: glm::vec2(1.0, 0.0), normal: glm::vec3(0.0, 1.0, 0.0)}
        ];

        // Indices for vertices order
        let indices: Vec<u32> = vec![
            0, 1, 2,
            0, 2, 3
        ];

        let mesh = Mesh::new(&gl, vertices, indices, vec![texture, specular_map]);

        Renderer {
            gl,
            shader,
            vertex_array,
            light_color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            light_position: glm::vec3(0.0, 0.0, 0.0),
            mesh
        }
    }

    pub fn begin(&mut self, camera: &PerspectiveCamera) {
        self.shader.bind(&self.gl);
        self.shader.upload_uniform_mat4(&self.gl, "projection_view",  &camera.projection_view());
        self.shader.upload_uniform_float4(&self.gl, "light_color",  glm::vec4(1.0, 1.0, 1.0, 1.0));
        self.shader.upload_uniform_float3(&self.gl, "light_position",  glm::vec3(1.0, 1.0, 1.0));
        self.shader.upload_uniform_float3(&self.gl, "camera_position",  camera.position());
    }

    pub fn end(&mut self) {
    }

    pub fn draw_mesh(&self, mesh: &Mesh, position: Vec3) {
        unsafe {
            self.shader.bind(&self.gl);
            self.shader.upload_uniform_integer1(&self.gl, "tex0", 0);
            self.shader.upload_uniform_integer1(&self.gl, "tex1", 1);
            self.shader.upload_uniform_float4(&self.gl, "light_color",  self.light_color);
            self.shader.upload_uniform_float3(&self.gl, "light_position",  self.light_position);

            let translate = glm::translate(&glm::identity(), &position);

            self.shader.upload_uniform_mat4(&self.gl, "model", &translate);

            Texture::bind(&self.gl, self.texture.get_renderer_id().unwrap(), 0);
            Texture::bind(&self.gl, self.specular_map.get_renderer_id().unwrap(), 1);

            self.vertex_array.bind(&self.gl);

            self.gl.draw_elements(glow::TRIANGLES, self.vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn draw_light(&mut self, position: Vec3, color: Vec4) {
        self.light_color = color;
        self.light_position = position;
    }

    pub fn borrow_context(&self) -> &glow::Context {
        &self.gl
    }

    pub fn clear(&self) {
        unsafe {
            self.gl.clear_color(0.07, 0.13, 0.17, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.clear_depth_f64(1.0);
            self.gl.depth_func(glow::LESS);
            self.gl.depth_mask(true);
            self.gl.enable(glow::DEPTH_TEST);
        }
    }
}