mod buffer;
mod shader;
pub mod texture;
pub mod camera;
pub mod layer;
pub mod event;
pub mod renderer;
pub mod app;
pub mod timer;

pub use nalgebra_glm as glm;
pub use glutin;
pub use egui;
pub use egui_glow;

#[cfg(test)]
mod tests {
    use crate::adze;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
