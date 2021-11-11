use adze_engine::app::App;
use adze_engine::renderer::Renderer;
use adze_engine::event::EventListener;
use adze_engine::layer::Layer;
use adze_engine::camera::OrthographicCamera;
use adze_engine::glm;
use adze_engine::glutin::event::VirtualKeyCode;

pub struct Sandbox {
    camera: OrthographicCamera
}

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {
            camera: OrthographicCamera::new(-1.0, 1.0, -1.0, 1.0)
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

        renderer.draw_square();

        renderer.draw_triangle();

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
