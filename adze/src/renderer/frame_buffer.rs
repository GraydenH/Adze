use glow::{HasContext, NativeBuffer};
use std::any::Any;
use core::mem;

pub struct FrameBuffer {
    renderer_id: glow::Framebuffer,
    color_attachment: glow::Texture,
    depth_attachment: glow::Texture,
    width: u32,
    height: u32,
    samples: u32,
    swap_chain_target: bool
}

impl FrameBuffer {
    pub fn new(
            gl: &glow::Context,
            width: u32,
            height: u32,
            samples: u32,
            swap_chain_target: bool
    ) -> FrameBuffer {
        unsafe {
            let frame_buffer = gl.create_framebuffer().unwrap();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(frame_buffer));

            let color_attachment = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(color_attachment));
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA8 as i32, width as i32, height as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, None);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(color_attachment), 0);

            let depth_attachment = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(depth_attachment));

            //gl.tex_storage_2d(glow::TEXTURE_2D, 1, glow::DEPTH24_STENCIL8, self.width as i32, self.height as i32);
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::DEPTH24_STENCIL8 as i32, width as i32, height as i32, 0, glow::DEPTH_STENCIL, glow::UNSIGNED_INT_24_8, None);
            // glTexImage2D(glow::TEXTURE_2D, 0, glow::DEPTH24_STENCIL8, m_Specification.Width, m_Specification.Height, 0,
            // 	glow::DEPTH_STENCIL, glow::UNSIGNED_INT_24_8, NULL);
            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::DEPTH_STENCIL_ATTACHMENT, glow::TEXTURE_2D, Some(depth_attachment), 0);

            //HZ_CORE_ASSERT(glCheckFramebufferStatus(glow::FRAMEBUFFER) == glow::FRAMEBUFFER_COMPLETE, "Framebuffer is incomplete!");

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            FrameBuffer {
                renderer_id: frame_buffer,
                color_attachment,
                depth_attachment,
                width,
                height,
                samples,
                swap_chain_target
            }
        }
    }

    pub fn invalidate(
        &self,
        gl: &glow::Context,
    ) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.renderer_id));
            gl.bind_texture(glow::TEXTURE_2D, Some(self.color_attachment));
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA8 as i32, self.width as i32, self.height as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, None);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(self.color_attachment), 0);

            gl.bind_texture(glow::TEXTURE_2D, Some(self.depth_attachment));

            // glTexImage2D(glow::TEXTURE_2D, 0, glow::DEPTH24_STENCIL8, m_Specification.Width, m_Specification.Height, 0,
            // 	glow::DEPTH_STENCIL, glow::UNSIGNED_INT_24_8, NULL);
            //gl.tex_storage_2d(glow::TEXTURE_2D, 1, glow::DEPTH24_STENCIL8, self.width as i32, self.height as i32);
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::DEPTH24_STENCIL8 as i32, self.width as i32, self.height as i32, 0, glow::DEPTH_STENCIL, glow::UNSIGNED_INT_24_8, None);
            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::DEPTH_STENCIL_ATTACHMENT, glow::TEXTURE_2D, Some(self.depth_attachment), 0);

            //HZ_CORE_ASSERT(glCheckFramebufferStatus(glow::FRAMEBUFFER) == glow::FRAMEBUFFER_COMPLETE, "Framebuffer is incomplete!");

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    pub fn get_renderer_id(&self) -> glow::Framebuffer {
        self.renderer_id
    }

    pub  fn unwrap(&self) -> u32 {
        #[cfg(not(target_os = "wasm"))]
        unsafe { mem::transmute(self.renderer_id) }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.renderer_id));
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_framebuffer(self.renderer_id);
        }
    }
}