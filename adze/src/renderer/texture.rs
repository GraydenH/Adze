use image::{GenericImageView, DynamicImage};
use glow::{HasContext, PixelUnpackData};

pub enum TextureType {
    Diffuse,
    Specular
}

pub struct Texture {
    renderer_id: glow::Texture,
    path: String,
    tiling: f32,
    width: u32,
    height: u32,
    data: Vec<u8>,
    internal_format: u32,
    data_format: u32,
    texture_type: TextureType
}

impl Texture {
    // https://www.reddit.com/r/rust/comments/7me7zr/using_image_crate_to_load_an_image_and_use_it_as/
    pub fn new(gl: &glow::Context, path: String, tiling: f32, texture_type: TextureType) -> Texture {
        match image::open(String::from(&path)) {
            Err(err) => panic!("Could not load image {}: {}", path, err),
            Ok(img) => unsafe {
                let (width, height) = img.dimensions();

                let (image, internal_format, data_format) = match img {
                    DynamicImage::ImageRgb8(img) => (img.into_raw(), glow::RGB8, glow::RGB),
                    DynamicImage::ImageRgba8(img) => (img.into_raw(), glow::RGBA8, glow::RGBA),
                    DynamicImage::ImageLuma8(img) => (img.into_raw(), glow::R8, glow::RED),
                    img => (img.to_rgb8().into_raw(), glow::RGB8, glow::RGB)
                };

                let renderer_id = gl.create_texture().unwrap();
                gl.bind_texture(glow::TEXTURE_2D, Some(renderer_id));
                gl.tex_storage_2d(glow::TEXTURE_2D, 1, internal_format, width as i32, height as i32);

                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);

                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

                gl.tex_sub_image_2d(glow::TEXTURE_2D, 0, 0, 0, width as i32, height as i32, data_format, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(image.as_slice()));
                gl.generate_mipmap(glow::TEXTURE_2D);

                Texture {
                    renderer_id,
                    path,
                    tiling,
                    width: 0,
                    height: 0,
                    data: vec![],
                    internal_format: 0,
                    data_format: 0,
                    texture_type
                }
            }
        }
    }

    pub fn get_renderer_id(&self) -> glow::Texture {
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

    pub fn from_dimensions(gl: &glow::Context, width: u32, height: u32, texture_type: TextureType) -> Self {
        unsafe {
            let internal_format = glow::RGBA8;
            let data_format = glow::RGBA;

            let renderer_id = gl.create_texture().unwrap();

            gl.bind_texture(glow::TEXTURE_2D, Some(renderer_id));

            gl.tex_storage_2d(glow::TEXTURE_2D, 1, internal_format, width as i32, height as i32);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

            Texture {
                renderer_id,
                path: "".to_string(),
                tiling: 1.0,
                width,
                height,
                data: vec![],
                internal_format,
                data_format,
                texture_type
            }
        }
    }

    pub fn from_data(gl: &glow::Context, data: Vec<u8>, width: u32, height: u32, internal_format: u32, data_format: u32, texture_type: TextureType) -> Self {
        unsafe {
            let renderer_id = gl.create_texture().unwrap();

            gl.bind_texture(glow::TEXTURE_2D, Some(renderer_id));

            gl.tex_storage_2d(glow::TEXTURE_2D, 1, internal_format, width as i32, height as i32);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

            gl.tex_sub_image_2d(glow::TEXTURE_2D, 0, 0, 0, width as i32, height as i32, data_format, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(data.as_slice()));
            Texture {
                renderer_id,
                path: "".to_string(),
                tiling: 1.0,
                width,
                height,
                data,
                internal_format,
                data_format,
                texture_type
            }
        }
    }

    pub fn set_data(&mut self, gl: &glow::Context, data: Vec<u8>) {
        self.data = data;
        Texture::bind(gl, self.renderer_id.unwrap(), 0);
        unsafe {
            gl.tex_sub_image_2d(glow::TEXTURE_2D, 0, 0, 0, self.width as i32, self.height as i32, self.data_format, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(self.data.as_slice()));
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn get_internal_format(&self) -> u32 {
        self.internal_format
    }

    pub fn set_internal_format(&mut self, internal_format: u32) {
        self.internal_format = internal_format;
    }

    pub fn get_data_format(&self) -> u32 {
        self.data_format
    }

    pub fn set_data_format(&mut self, data_format: u32) {
        self.data_format = data_format;
    }

    pub fn bind(gl: &glow::Context, renderer_id: glow::Texture, slot: u32) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + slot);
            gl.bind_texture(glow::TEXTURE_2D, Some(renderer_id));
        }
    }

    fn unbind(gl: &glow::Context, slot: u32) {
        unsafe {
            gl.active_texture(slot);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}

