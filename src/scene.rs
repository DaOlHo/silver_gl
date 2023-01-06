use super::GlError;

// TODO: turn this into a scene graph to make it function more like a normal engine?
pub trait Scene {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), GlError>;
    fn draw(&mut self) -> Result<(), GlError>;
}