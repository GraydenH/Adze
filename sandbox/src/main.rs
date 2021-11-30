use adze::app::App;
use adze::renderer::Renderer;
use adze::layer::Layer;
use adze::glm;
use adze::glutin::event::VirtualKeyCode;
use adze::glm::{Mat4, Vec2};
use adze::texture::Texture;
use adze::timer::Timer;
use adze::renderer::camera::WasdCameraController;
use adze::app::event::EventListener;

pub struct Sandbox {
    camera_controller: WasdCameraController,
    checker_board_texture: Texture,
    cherno_logo_texture: Texture
}

impl Sandbox {
    pub fn new() -> Self {
        let _timer = Timer::new("SandBox::new");

        let checker_board_texture = Texture::new(String::from("sandbox/assets/textures/Checkerboard.png"), 10.0);
        let cherno_logo_texture = Texture::new(String::from("sandbox/assets/textures/ChernoLogo.png"), 1.0);

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
        let _timer = Timer::new("SandBox::on_tick");

        self.camera_controller.on_tick();

        self.camera_controller.get_camera().recalculate_matrix();

        renderer.clear();

        renderer.begin(&self.camera_controller.get_camera());

        renderer.draw_quad(glm::vec3(0.0, 0.0, 0.0), glm::vec2(4.0, 4.0), &mut self.checker_board_texture);

        for y in 0..20 {
            for x in 0..20 {
                let pos = glm::vec3(-0.5 + (x as f32) * 0.11, -0.5 + (y as f32) * 0.11, 0.0);
                renderer.draw_flat_color_quad(pos, glm::vec3(0.1, 0.1, 0.1),glm::vec4((x as f32) / 20.0, 0.0, (y as f32) / 20.0, 1.0));
            }
        }

        renderer.draw_quad(glm::vec3(0.1, 0.1, 0.1), glm::vec2(0.1, 0.1), &mut self.cherno_logo_texture);

        renderer.end();
    }

    fn on_window_resize(&mut self, width: u32, height: u32) {
        self.camera_controller.on_window_resize(width, height);
    }

    fn on_mouse_scroll(&mut self, delta: Vec2) -> bool {
        self.camera_controller.on_mouse_scroll(delta)
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
