mod buffer;
mod shader;
pub mod camera;
pub mod layer;
pub mod event;
pub mod renderer;
pub mod app;

pub use nalgebra_glm as glm;
pub use glutin;

#[cfg(test)]
mod tests {
    use crate::adze;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}