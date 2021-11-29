use adze_engine::app::App;
use adze_engine::renderer::Renderer;
use adze_engine::event::EventListener;
use adze_engine::layer::Layer;
use adze_engine::camera::{OrthographicCamera, WasdCameraController};
use adze_engine::glm;
use adze_engine::glutin::event::VirtualKeyCode;
use adze_engine::glm::{Mat4, Vec2};
use adze_engine::texture::Texture;
use adze_engine::timer::Timer;

pub struct Editor {
}

impl Editor {
    pub fn new() -> Self {
        let _timer = Timer::new("Editor::new");
        Editor {
        }
    }
}

impl EventListener for Editor {
    fn on_tick(&mut self, _renderer: &mut Renderer) {
        let _timer = Timer::new("Editor::on_tick");
    }

}

impl Layer for Editor {
    fn on_attach(&mut self) {

    }

    fn on_detach(&mut self) {

    }
}

fn main() {
    let mut app = App::new("sandbox");
    let sandbox = Box::new(Editor::new());
    app.push_layer(sandbox);
    app.run();
}
