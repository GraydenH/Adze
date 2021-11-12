pub struct Texture {
    renderer_id: Option<glow::Texture>,
    path: String
}

impl Texture {
    pub fn new(path: String) -> Texture {
        Texture {
            renderer_id: None,
            path
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
}

