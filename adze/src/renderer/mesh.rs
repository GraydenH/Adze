use crate::renderer::texture::Texture;
use crate::renderer::Vertex;
use crate::renderer::buffer::{BufferLayout, VertexArray, BufferElement, IndexBuffer, VertexBuffer, ShaderDataType};

pub struct Mesh {
    vertex_array: VertexArray,
    textures: Vec<Texture>
}

impl Mesh {
    pub fn new(gl: &glow::Context, vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        unsafe {
            let layout = BufferLayout::new(
                vec![
                    BufferElement::new("aposition".parse().unwrap(), ShaderDataType::Float3, false),
                    BufferElement::new("acolor".parse().unwrap(), ShaderDataType::Float3, false),
                    BufferElement::new("atexture_coordinate".parse().unwrap(), ShaderDataType::Float2, false),
                    BufferElement::new("anormal".parse().unwrap(), ShaderDataType::Float3, false),
                ]
            );

            let index_buffer = IndexBuffer::new(&gl, indices);
            let vertex_buffer = VertexBuffer::new(&gl, vertices, layout);
            let vertex_array = VertexArray::new(&gl, index_buffer, vertex_buffer);

            Mesh {
                vertex_array,
                textures
            }
        }
    }
}