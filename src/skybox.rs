use super::{Model, ShaderProgram, GlError, gl};

pub struct Skybox {
    pub model: Model
}

impl Skybox {
    pub fn draw(&self, shader_program: &ShaderProgram) -> Result<(), GlError> {
        unsafe {
            // Change depth func so test values pass when they are equal to the buffer's content
            gl::DepthFunc(gl::LEQUAL);

            shader_program.use_program();
            self.model.draw(shader_program)?;

            gl::DepthFunc(gl::LESS);
        }

        Ok(())
    }
}