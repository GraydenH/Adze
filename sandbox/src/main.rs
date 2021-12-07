use adze::app::App;
use adze::renderer::Renderer;
use adze::glm;
use adze::glm::{Vec2};
use adze::renderer::camera::{WasdCameraController, FlyingCameraController};
use adze::app::event::EventListener;
use adze::app::layer::Layer;
use adze::renderer::texture::Texture;
use adze::app::timer::Timer;

pub struct Sandbox {
    camera_controller: FlyingCameraController,
    rotation: f32
}

impl Sandbox {
    pub fn new() -> Self {
        let _timer = Timer::new("SandBox::new");

        let mut camera_controller = FlyingCameraController::new(0.785398, (800.0 / 600.0), 0.1, 100.0);

        Sandbox {
            camera_controller,
            rotation: 0.0
        }
    }
}

impl EventListener for Sandbox {
    fn on_tick(&mut self, renderer: &mut Renderer) {
        let _timer = Timer::new("SandBox::on_tick");

        self.rotation += 0.01;

        self.camera_controller.recalculate_matrix();

        renderer.clear();
        renderer.begin(&self.camera_controller.camera());
        renderer.draw(self.rotation);
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
