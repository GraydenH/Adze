use glow::{HasContext, Buffer};
use crate::renderer::buffer;
use crate::renderer::QuadVertex;

#[derive(Clone, Copy)]
pub enum ShaderDataType {
    None = 0,
    Float1,
    Float2,
    Float3,
    Float4,
    Matrix3,
    Matrix4,
    Integer1,
    Integer2,
    Integer3,
    Integer4,
    Boolean
}

pub struct BufferElement {
    name: String,
    offset: i32,
    size: i32,
    data_type: ShaderDataType,
    normalized: bool
}


fn get_shader_data_type_size(data_type: ShaderDataType) -> i32 {
    return match data_type {
        ShaderDataType::Float1 => 4,
        ShaderDataType::Float2 => 4 * 2,
        ShaderDataType::Float3 => 4 * 3,
        ShaderDataType::Float4 => 4 * 4,
        ShaderDataType::Matrix3 => 3 * 3 * 3,
        ShaderDataType::Matrix4 => 4 * 4 * 4,
        ShaderDataType::Integer1 => 4,
        ShaderDataType::Integer2 => 4 * 2,
        ShaderDataType::Integer3 => 4 * 3,
        ShaderDataType::Integer4 => 4 * 4,
        ShaderDataType::Boolean => 1,
        _ => 0
    };
}

impl BufferElement {
    pub fn new(name: String, data_type: ShaderDataType, normalized: bool) -> BufferElement {
        BufferElement {
            name,
            data_type,
            size: get_shader_data_type_size(data_type),
            normalized,
            offset: 0
        }
    }

    fn get_component_count(&self) -> i32 {
        return match self.data_type {
            ShaderDataType::Float1 => 1,
            ShaderDataType::Float2 => 2,
            ShaderDataType::Float3 => 3,
            ShaderDataType::Float4 => 4,
            ShaderDataType::Matrix3 => 3 * 3,
            ShaderDataType::Matrix4 => 4 * 4,
            ShaderDataType::Integer1 => 1,
            ShaderDataType::Integer2 => 2,
            ShaderDataType::Integer3 => 3,
            ShaderDataType::Integer4 => 4,
            ShaderDataType::Boolean => 1,
            _ => 0 // error?
        };
    }
}

fn to_opengl_type(data_type: ShaderDataType) -> u32 {
    return match data_type {
        ShaderDataType::Float1 => glow::FLOAT,
        ShaderDataType::Float2 => glow::FLOAT,
        ShaderDataType::Float3 => glow::FLOAT,
        ShaderDataType::Float4 => glow::FLOAT,
        ShaderDataType::Matrix3 => glow::FLOAT,
        ShaderDataType::Matrix4 => glow::FLOAT,
        ShaderDataType::Integer1 => glow::INT,
        ShaderDataType::Integer2 => glow::INT,
        ShaderDataType::Integer3 => glow::INT,
        ShaderDataType::Integer4 => glow::INT,
        ShaderDataType::Boolean => glow::BOOL,
        _ => 0 // throw error?
    };
}

pub struct BufferLayout {
    elements: Vec<BufferElement>,
    stride: i32
}

impl BufferLayout {
    pub fn new(mut elements: Vec<BufferElement>) -> BufferLayout {
        let mut offset: i32 = 0;
        let mut stride: i32 = 0;

        for element in elements.iter_mut() {
            element.offset = offset;
            offset += element.size;
            stride += element.size;
        }

        BufferLayout {
            elements,
            stride
        }
    }
}

pub struct VertexBuffer {
    vertices: Vec<f32>,
    layout: BufferLayout,
    renderer_id: Buffer
}

impl VertexBuffer {
    pub fn new(gl: &glow::Context, vertices: Vec<f32>, layout: BufferLayout) -> VertexBuffer {
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
                layout,
                renderer_id
            }
        }
    }

    pub fn from_size(gl: &glow::Context, size: i32, layout: BufferLayout) -> VertexBuffer {
        unsafe {
            // Create a Vertex Buffer Object and copy the vertex data to it
            let renderer_id = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(renderer_id));
            gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                size,
                glow::DYNAMIC_DRAW,
            );
            VertexBuffer {
                vertices: vec![],
                layout,
                renderer_id
            }
        }
    }

    fn set_vertices(&mut self, gl: &glow::Context, vertices: &Vec<QuadVertex>) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.renderer_id));
            let vertices_u8: &[u8] = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<QuadVertex>(),
            );
            gl.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                0,
                vertices_u8
            );
        }
    }

    fn get_vertices(&mut self) -> &Vec<f32> {
        &self.vertices
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

    pub fn get_indices_len(&self) -> usize {
        self.indices.len()
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
    vertex_buffer: buffer::VertexBuffer,
    index_buffer: buffer::IndexBuffer,
    renderer_id: glow::VertexArray
}

impl VertexArray {
    pub fn new(gl: &glow::Context, index_buffer: buffer::IndexBuffer, vertex_buffer: buffer::VertexBuffer) -> VertexArray {
        unsafe {
            // Create Vertex Array Object
            let renderer_id = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(renderer_id));
            index_buffer.bind(gl);
            vertex_buffer.bind(gl);
            for (index, element) in vertex_buffer.layout.elements.iter().enumerate() {
                // Specify the layout of the vertex data
                gl.enable_vertex_attrib_array(index as u32);
                gl.vertex_attrib_pointer_f32(
                    index as u32,
                    element.get_component_count(),
                    to_opengl_type(element.data_type),
                    false,
                    vertex_buffer.layout.stride,
                    element.offset,
                );
            }
            VertexArray {
                vertex_buffer,
                index_buffer,
                renderer_id
            }
        }
    }

    pub fn set_vertices(&mut self, gl: &glow::Context, vertices: &Vec<QuadVertex>) {
        self.vertex_buffer.set_vertices(gl, vertices);
    }

    pub fn get_indices_len(&self) -> usize {
        self.index_buffer.get_indices_len()
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