use std::{fmt::Display, error::Error, ffi::NulError, io};

use super::ShaderCompileType;

#[derive(Debug)]
pub enum GlError{
    CStringError(NulError),
    UniformNotFound(String, u32),
    ShaderCompileError(ShaderCompileType, u32, String),
    IoError(io::Error),
    UniformInvalidIndex(String, u32),
    FramebufferNotComplete(u32),
    UniformBufferMissing,
    CannotResize(u32),
    IncorrectSize(String)
}

impl Display for GlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlError::CStringError(nul_error) => write!(f, "{}", nul_error),
            GlError::UniformNotFound(uniform, id) => 
                write!(f, "Uniform '{}' was not found in shader {}", uniform, id),
            GlError::ShaderCompileError(type_, id, error) =>
                write!(f, "Shader '{}' with ID {} failed to compile:\n{}", type_, id, error),
            GlError::IoError(io_error) => write!(f, "{}", io_error),
            GlError::UniformInvalidIndex(ub_name, id) => {
                write!(f, "Uniform block '{}' was not found in shader {}", ub_name, id)
            },
            GlError::FramebufferNotComplete(id) => {
                write!(f, "Framebuffer '{}' is not complete", id)
            },
            GlError::UniformBufferMissing => write!(f, "Uniform buffer is not present"),
            GlError::CannotResize(id) => write!(f, "Cannot resize texture '{}'", id),
            GlError::IncorrectSize(msg) => write!(f, "Icorrect size: {}", msg)
        }
    }
}

impl Error for GlError {}

impl From<NulError> for GlError {
    fn from(err: NulError) -> Self {
        GlError::CStringError(err)
    }
}

impl From<io::Error> for GlError {
    fn from(err: io::Error) -> Self {
        GlError::IoError(err)
    }
}