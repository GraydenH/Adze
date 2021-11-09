use adze_engine::app::App;
use adze_engine::renderer::Renderer;
use adze_engine::event::EventListener;
use adze_engine::layer::Layer;

pub struct Sandbox {
}

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {
        }
    }
}

impl EventListener for Sandbox {
    fn on_tick(&mut self, renderer: &Renderer) {
        renderer.clear();

        renderer.begin();

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
