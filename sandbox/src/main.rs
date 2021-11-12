use adze_engine::app::App;
use adze_engine::renderer::Renderer;
use adze_engine::event::EventListener;
use adze_engine::layer::Layer;
use adze_engine::camera::{OrthographicCamera, WasdCameraController};
use adze_engine::glm;
use adze_engine::glutin::event::VirtualKeyCode;
use adze_engine::glm::Mat4;
use adze_engine::texture::Texture;

pub struct Sandbox {
    camera_controller: WasdCameraController,
    checker_board_texture: Texture,
    cherno_logo_texture: Texture
}

impl Sandbox {
    pub fn new() -> Self {
        let checker_board_texture = Texture::new(String::from("sandbox/assets/textures/Checkerboard.png"));
        let cherno_logo_texture = Texture::new(String::from("sandbox/assets/textures/ChernoLogo.png"));

        let mut camera_controller = WasdCameraController::new(1.0);
        camera_controller.set_translation_speed(0.1);

        Sandbox {
            camera_controller,
            checker_board_texture,
            cherno_logo_texture
        }
    }
}

impl EventListener for Sandbox {
    fn on_tick(&mut self, renderer: &mut Renderer) {
        self.camera_controller.on_tick();

        self.camera_controller.get_camera().recalculate_matrix();

        renderer.clear();

        renderer.begin(&self.camera_controller.get_camera());

        renderer.draw_quad_with_texture(&glm::identity(), &mut self.checker_board_texture);

        let scale = glm::scale(&glm::identity(), &glm::vec3(0.1, 0.1, 0.1));

        for y in 0..20 {
            for x in 0..20 {
                let pos = glm::vec3((x as f32) * 0.11, (y as f32) * 0.11, 0.0);
                let transform = glm::translate(&glm::identity(), &pos) * scale;
                renderer.draw_flat_color_quad(&transform, &glm::vec3((x as f32) * 0.05, (x as f32) * 0.05, (x + y) as f32 * 0.01));
            }
        }

        renderer.draw_quad_with_texture(&glm::identity(), &mut self.cherno_logo_texture);

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
