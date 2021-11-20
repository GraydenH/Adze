use image::{GenericImageView, DynamicImage};
use glow::{HasContext, PixelUnpackData};

pub struct Texture {
    renderer_id: Option<glow::Texture>,
    path: String,
    tiling: f32,
}

impl Texture {
    pub fn new(path: String, tiling: f32) -> Texture {
        Texture {
            renderer_id: None,
            path,
            tiling
        }
    }

    pub fn set_renderer_id(&mut self, renderer_id: glow::Texture) {
        self.renderer_id = Some(renderer_id);
    }

    pub fn get_renderer_id(&self) -> Option<glow::Texture> {
        self.renderer_id
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn set_tiling(&mut self, tiling: f32) {
        self.tiling = tiling;
    }

    pub fn get_tiling(&self) -> f32 {
        self.tiling
    }

    // https://www.reddit.com/r/rust/comments/7me7zr/using_image_crate_to_load_an_image_and_use_it_as/
    pub(crate) fn init(&mut self, gl: &glow::Context) {
        match image::open(String::from(self.get_path())) {
            Err(err) => panic!("Could not load image {}: {}", self.get_path(), err),
            Ok(img) => unsafe {
                let (width, height) = img.dimensions();

                let (image, internal_format, data_format) = match img {
                    DynamicImage::ImageRgb8(img) => (img.into_raw(), glow::RGB8, glow::RGB),
                    DynamicImage::ImageRgba8(img) => (img.into_raw(), glow::RGBA8, glow::RGBA),
                    img => (img.to_rgb8().into_raw(), glow::RGB8, glow::RGB)
                };

                let renderer_id = gl.create_texture().unwrap();
                gl.bind_texture(glow::TEXTURE_2D, Some(renderer_id));
                gl.tex_storage_2d(glow::TEXTURE_2D, 1, internal_format, width as i32, height as i32);

                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);

                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

                gl.tex_sub_image_2d(glow::TEXTURE_2D, 0, 0, 0, width as i32, height as i32, data_format, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(image.as_slice()));

                self.set_renderer_id(renderer_id);
            }
        }
    }

    pub(crate) fn bind(&self, gl: &glow::Context, slot: u32) {
        unsafe {
            gl.active_texture(slot);
            gl.bind_texture(glow::TEXTURE_2D, self.get_renderer_id());
        }
    }

    fn unbind(gl: &glow::Context, slot: u32) {
        unsafe {
            gl.active_texture(slot);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}

