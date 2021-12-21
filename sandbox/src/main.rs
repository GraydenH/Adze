use adze::app::App;
use adze::renderer::Renderer;
use adze::glm;
use adze::glm::{Vec2};
use adze::renderer::camera::{WasdCameraController, FlyingCameraController};
use adze::app::event::EventListener;
use adze::app::layer::Layer;
use adze::renderer::texture::Texture;
use adze::app::timer::Timer;
use adze::renderer::mesh::Mesh;

pub struct Sandbox {
    camera_controller: FlyingCameraController,
    width: u32,
    height: u32,
    mesh: Mesh,
}

impl Sandbox {
    pub fn new(width: u32, height: u32) -> Self {
        let _timer = Timer::new("SandBox::new");

        let mut camera_controller = FlyingCameraController::new((width as f32 / height as f32), 45.0, 0.1, 100.0, width, height);

        Sandbox {
            camera_controller,
            width,
            height,
        }
    }
}

impl EventListener for Sandbox {
    fn on_tick(&mut self, renderer: &mut Renderer) {
        let _timer = Timer::new("SandBox::on_tick");

        self.camera_controller.on_tick();
        self.camera_controller.recalculate_matrix();

        renderer.clear();
        renderer.begin(&self.camera_controller.camera());
        renderer.draw_light(glm::vec3(0.5, 0.5, 0.5), glm::vec4(1.0, 1.0, 1.0, 1.0));
        renderer.draw_mesh(&self.mesh, glm::vec3(0.0, 0.0, 0.0));
        renderer.end();
    }

    fn on_mouse_move(&mut self, position: Vec2) -> bool {
        self.camera_controller.on_mouse_move(position);
        false
    }
}

impl Layer for Sandbox {
    fn on_attach(&mut self) {

    }

    fn on_detach(&mut self) {

    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut app = App::new("sandbox", width, height);
    let sandbox = Box::new(Sandbox::new(width, height));
    app.push_layer(sandbox);
    app.run();
}
