use adze_engine::app::App;
use adze_engine::renderer::Renderer;
use adze_engine::event::EventListener;
use adze_engine::layer::Layer;
use adze_engine::camera::OrthographicCamera;
use adze_engine::glm;
use adze_engine::glutin::event::VirtualKeyCode;
use adze_engine::glm::Mat4;
use adze_engine::texture::Texture;

pub struct Sandbox {
    camera: OrthographicCamera,
    texture: Texture
}

impl Sandbox {
    pub fn new() -> Self {
        let texture = Texture::new(String::from("sandbox/assets/textures/Checkerboard.png"));
        Sandbox {
            camera: OrthographicCamera::new(-1.0, 1.0, -1.0, 1.0),
            texture
        }
    }

    fn move_camera(&mut self) {
        if App::is_key_pressed(VirtualKeyCode::A) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(-0.01, 0.0, 0.0));
        } else if App::is_key_pressed(VirtualKeyCode::D) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(0.01, 0.0, 0.0));
        } else if App::is_key_pressed(VirtualKeyCode::W) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(0.0, 0.01, 0.0));
        } else if App::is_key_pressed(VirtualKeyCode::S) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(0.0, -0.01, 0.0));
        }
    }
}

impl EventListener for Sandbox {
    fn on_tick(&mut self, renderer: &mut Renderer) {
        self.move_camera();
        self.camera.set_rotation(0.0);

        self.camera.recalculate_matrix();

        renderer.clear();

        renderer.begin(&self.camera);

        renderer.draw_quad_with_texture(&glm::identity(), &mut self.texture);

        let scale = glm::scale(&glm::identity(), &glm::vec3(0.1, 0.1, 0.1));

        for y in 0..20 {
            for x in 0..20 {
                let pos = glm::vec3((x as f32) * 0.11, (y as f32) * 0.11, 0.0);
                let transform = glm::translate(&glm::identity(), &pos) * scale;
                renderer.draw_flat_color_quad(&transform, &glm::vec3((x as f32) * 0.05, (x as f32) * 0.05, (x + y) as f32 * 0.01));
            }
        }

        renderer.end();
    }
}

impl Layer for Sandbox {
    fn on_attach(&mut self) {

    }

    fn on_detach(&mut self) {

    }
}

fn main() {
    let mut app = App::new("sandbox");
    let sandbox = Box::new(Sandbox::new());
    app.push_layer(sandbox);
    app.run();
}
