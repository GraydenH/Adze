use adze::app::App;
use adze::renderer::Renderer;
use adze::event::EventListener;
use adze::layer::Layer;
use adze::renderer::camera::{OrthographicCamera, WasdCameraController};
use adze::{glm, egui};
use adze::glutin::event::VirtualKeyCode;
use adze::glm::{Mat4, Vec2};
use adze::texture::Texture;
use adze::timer::Timer;
use adze::egui_glow::EguiGlow;

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
    fn on_ui_update(&mut self, egui: &EguiGlow) {
        let _timer = Timer::new("Editor::on_tick");
        egui::SidePanel::left("my_side_panel").show(egui.ctx(), |ui| {
            ui.heading("Hello World!");
            if ui.button("Quit").clicked() {
                //quit = true;
            }
        });
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
