use adze::app::App;
use adze::renderer::Renderer;
use adze::event::EventListener;
use adze::layer::Layer;
use adze::camera::{OrthographicCamera, WasdCameraController};
use adze::glm;
use adze::glutin::event::VirtualKeyCode;
use adze::glm::{Mat4, Vec2};
use adze::texture::Texture;
use adze::timer::Timer;

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
