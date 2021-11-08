use core::mem;
use glow::{HasContext, Buffer};

pub struct VertexBuffer {
    vertices: Vec<f32>,
    renderer_id: Buffer
}

impl VertexBuffer {
    pub fn new(gl: &glow::Context, vertices: Vec<f32>) -> VertexBuffer {
        unsafe {
            let renderer_id = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(renderer_id));

            let vertices_u8: &[u8] = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);
            VertexBuffer {
                vertices,
                renderer_id
            }
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.renderer_id));
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }
    }
}

pub struct IndexBuffer {
    indices: Vec<u32>,
    renderer_id: Buffer
}

impl IndexBuffer {
    pub fn new(gl: &glow::Context, indices: Vec<u32>) -> IndexBuffer {
        unsafe {
            // Create a Vertex Buffer Object and copy the vertex data to it
            let renderer_id = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(renderer_id));
            let indices_u8: &[u8] = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * core::mem::size_of::<u32>(),
            );
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices_u8, glow::STATIC_DRAW);
            IndexBuffer {
                indices,
                renderer_id
            }
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.renderer_id));
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }
}

pub struct VertexArray {
    vertex_buffers: Vec<VertexBuffer>,
    renderer_id: glow::VertexArray
}

impl VertexArray {
    pub fn new(gl: &glow::Context) -> VertexArray {
        unsafe {
            // Create Vertex Array Object
            let renderer_id = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(renderer_id));
            VertexArray {
                vertex_buffers: vec![],
                renderer_id
            }
        }
    }

    pub fn add_vertex_buffer(&mut self, gl: &glow::Context, buffer: VertexBuffer) {
        self.bind(gl);
        buffer.bind(gl);

        unsafe {
            // Specify the layout of the vertex data
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                28,
                0,
            );

            // Specify the layout of the vertex data
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                4,
                glow::FLOAT,
                false,
                28,
                12,
            );

            self.vertex_buffers.push(buffer);
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.renderer_id));
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(None);
        }
    }
}