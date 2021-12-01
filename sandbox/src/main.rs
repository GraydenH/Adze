use adze::app::App;
use adze::renderer::Renderer;
use adze::{glm, egui};
use adze::glm::{Vec2};
use adze::renderer::camera::WasdCameraController;
use adze::app::event::EventListener;
use adze::app::layer::Layer;
use adze::renderer::texture::Texture;
use adze::app::timer::Timer;
use adze::renderer::frame_buffer::FrameBuffer;
use adze::egui_glow::EguiGlow;
use adze::egui::epaint::TextureId::User;

pub struct Sandbox {
    frame_buffer: Option<FrameBuffer>,
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
            frame_buffer: None,
            camera_controller,
            checker_board_texture,
            cherno_logo_texture
        }
    }
}

impl EventListener for Sandbox {
    fn on_tick(&mut self, renderer: &mut Renderer) {
        if self.frame_buffer.is_none() {
            self.frame_buffer = Some(FrameBuffer::new(renderer.borrow_context(), 1200, 720, 1 ,false));
        } else {
            self.frame_buffer.as_ref().unwrap().invalidate(renderer.borrow_context());
        }

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
