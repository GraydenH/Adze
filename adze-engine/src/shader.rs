use glow::HasContext;

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