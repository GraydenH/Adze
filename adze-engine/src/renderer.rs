use crate::buffer::VertexArray;
use core::ptr;
use glow::HasContext;
use crate::buffer;

pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {

        }
    }

    pub fn begin(&self) {

    }

    pub fn end(&self) {

    }

    pub fn clear(&self, gl: &glow::Context) {
        // Clear the screen to black
        unsafe {
            gl.clear_color(0.3, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&self, gl: &glow::Context, vertex_array: &buffer::VertexArray) {
        vertex_array.bind(gl);
        unsafe {
            gl.draw_elements(glow::TRIANGLES, vertex_array.get_indices_len() as i32, glow::UNSIGNED_INT, 0);
        }
    }
}