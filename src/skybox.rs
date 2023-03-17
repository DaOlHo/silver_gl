use crate::MultiBindModel;

use super::{ModelTrait, ShaderProgram, GlError, gl};

pub struct Skybox {
    pub model: MultiBindModel // Only one skybox at a time is presumed to exist
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