use adze::app::App;
use adze::renderer::Renderer;
use adze::app::event::EventListener;
use adze::app::layer::Layer;
use adze::renderer::camera::{OrthographicCamera, WasdCameraController};
use adze::{glm, egui};
use adze::glutin::event::VirtualKeyCode;
use adze::glm::{Mat4, Vec2};
use adze::renderer::texture::Texture;
use adze::app::timer::Timer;
use adze::renderer::frame_buffer::FrameBuffer;
use adze::egui_glow::EguiGlow;
use adze::egui::Image;
use adze::egui::paint::TextureId::User;

pub struct Editor {
    frame_buffer: Option<FrameBuffer>
}

impl Editor {
    pub fn new() -> Self {
        let _timer = Timer::new("Editor::new");
        Editor {
            frame_buffer: None
        }
    }
}

impl EventListener for Editor {
    fn on_tick(&mut self, renderer: &mut Renderer) {
        if self.frame_buffer.is_none() {
            self.frame_buffer = Some(FrameBuffer::new(renderer.borrow_context(), 1200, 720, 1 ,false));
        }
    }

    fn on_ui_update(&mut self, egui: &EguiGlow) {
        let _timer = Timer::new("Editor::on_tick");
        egui::SidePanel::left("my_side_panel").show(egui.ctx(), |ui| {
            ui.heading("Hello World!");
            if ui.button("Quit").clicked() {
                //quit = true;
            }
        });

        if self.frame_buffer.is_some() {
            egui::Image::new(User(self.frame_buffer.as_ref().unwrap().unwrap() as u64), egui::Vec2::new(1200.0, 720.0));
        }
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
