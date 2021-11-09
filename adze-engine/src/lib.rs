mod buffer;
mod shader;
mod button;
pub mod layer;
pub mod event;
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