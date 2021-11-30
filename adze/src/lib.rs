pub use egui;
pub use egui_glow;
pub use glutin;
pub use nalgebra_glm as glm;

pub mod renderer;
pub mod app;

#[cfg(test)]
mod tests {
    use crate::adze;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
