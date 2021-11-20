use glow::HasContext;
use nalgebra_glm::Mat4;
use crate::glm::{Vec3, Vec2, Vec4, Mat3};

fn compile_shader(gl: &glow::Context, src: &str, ty: u32) -> glow::Shader {
    unsafe {
        let shader = gl.create_shader(ty).unwrap();
        gl.shader_source(shader, src);
        gl.compile_shader(shader);

        // Get the compile status
        let mut status = gl.get_shader_compile_status(shader);

        // Fail on error
        if !status {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        shader
    }
}

fn link_program(gl: &glow::Context, vs: glow::Shader, fs: glow::Shader) -> glow::Program {
    unsafe {
        let program = gl.create_program().unwrap();
        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);
        // Get the link status
        let mut status = gl.get_program_link_status(program);

        // Fail on error
        if !status {
            panic!("{}", gl.get_program_info_log(program));
        }
        program
    }
}

pub struct Shader {
    renderer_id: glow::Program
}

impl Shader {
    pub fn new(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Shader {
        let vs = compile_shader(gl, vertex_src, glow::VERTEX_SHADER);
        let fs = compile_shader(gl, fragment_src, glow::FRAGMENT_SHADER);
        let renderer_id = link_program(gl, vs, fs);
        Shader {
            renderer_id
        }
    }

    pub fn upload_uniform_integer1(&self, gl: &glow::Context, name: &str, value: i32) {
        unsafe {
            let location = gl.get_uniform_location(self.renderer_id, name).unwrap();
            gl.uniform_1_i32(Some(&location), value);
        }
    }

    pub fn upload_uniform_float1(&self, gl: &glow::Context, name: &str, value: f32) {
        unsafe {
            let location = gl.get_uniform_location(self.renderer_id, name).unwrap();
            gl.uniform_1_f32(Some(&location), value);
        }
    }

    pub fn upload_uniform_float2(&self, gl: &glow::Context, name: &str, value: Vec2) {
        unsafe {
            let location = gl.get_uniform_location(self.renderer_id, name).unwrap();
            gl.uniform_2_f32(Some(&location), value.x, value.y);
        }
    }

    pub fn upload_uniform_float3(&self, gl: &glow::Context, name: &str, value: Vec3) {
        unsafe {
            let location = gl.get_uniform_location(self.renderer_id, name).unwrap();
            gl.uniform_3_f32(Some(&location), value.x, value.y, value.z);
        }
    }

    pub fn upload_uniform_float4(&self, gl: &glow::Context, name: &str, value: Vec4) {
        unsafe {
            let location = gl.get_uniform_location(self.renderer_id, name).unwrap();
            gl.uniform_4_f32(Some(&location), value.x, value.y, value.z, value.w);
        }
    }

    pub fn upload_uniform_matrix3(&self, gl: &glow::Context, name: &str, matrix: &Mat3) {
        unsafe {
            let location = gl.get_uniform_location(self.renderer_id, name).unwrap();
            gl.uniform_matrix_3_f32_slice(Some(&location), false, matrix.as_slice());
        }
    }

    pub fn upload_uniform_mat4(&self, gl: &glow::Context, name: &str, matrix: &Mat4) {
        unsafe {
            let location = gl.get_uniform_location(self.renderer_id, name).unwrap();
            gl.uniform_matrix_4_f32_slice(Some(&location), false, matrix.as_slice());
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.renderer_id));
        }
    }

    fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(None);
        }
    }
}