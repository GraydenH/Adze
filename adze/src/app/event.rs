
use crate::renderer::Renderer;
use glutin::event::VirtualKeyCode;
use crate::glm::Vec2;
use egui_glow::EguiGlow;

// https://github.com/mathiasmagnusson
pub trait EventListener {
    fn on_tick(&mut self, renderer: &mut Renderer) {}
    fn on_ui_update(&mut self, egui: &EguiGlow) {}
    fn on_window_closed(&mut self) {}
    fn on_window_resize(&mut self, _width: u32, _height: u32) {}
    fn on_key_press(&mut self, _button: VirtualKeyCode, _repeat: bool) -> bool {
        false
    }
    fn on_key_release(&mut self, _button: VirtualKeyCode) -> bool {
        false
    }
    fn on_char_written(&mut self, _which: char) -> bool {
        false
    }
    fn on_mouse_press(&mut self, _button: VirtualKeyCode) -> bool {
        false
    }
    fn on_mouse_release(&mut self, _button: VirtualKeyCode) -> bool {
        false
    }
    fn on_mouse_move(&mut self, _position: Vec2) -> bool {
        false
    }
    fn on_mouse_scroll(&mut self, _delta: Vec2) -> bool {
        false
    }
}