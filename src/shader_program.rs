use std::{ffi::CString, fmt::Display};
use std::ptr;
use cgmath::{Vector3, Array, Matrix4, Matrix, Vector4};
use super::error::GlError;

pub struct ShaderProgram { id: u32 }

impl ShaderProgram {
    pub fn new(shader_bundle: ShaderCodeBundle) -> Result<ShaderProgram, GlError> {
        let mut shader_program = ShaderProgram { id: 0 };

        shader_program.compile_program(shader_bundle)?;

        Ok(shader_program)
    }

    pub fn compile_program(&mut self, shader_bundle: ShaderCodeBundle) -> Result<(), GlError> {
        let mut shader_ids = Vec::new();

        for (code, type_) in shader_bundle.get_vec() {
            if let Some(code) = code {
                shader_ids.push(ShaderProgram::compile_shader(code, type_)?);
            }
        }

        let shader_program_id;

        unsafe {
            shader_program_id = gl::CreateProgram();

            println!("DEBUG::SHADER::PROGRAM::ATTACHING_SHADERS");

            for id in shader_ids.iter() {
                gl::AttachShader(shader_program_id, *id);
            }

            println!("DEBUG::SHADER::PROGRAM::COMPILING_PROGRAM");

            gl::LinkProgram(shader_program_id);
            ShaderProgram::check_compile_errors(shader_program_id, ShaderCompileType::Program)?;

            println!("DEBUG::SHADER::PROGRAM::COMPILATION_COMPLETE");

            for id in shader_ids.iter() {
                gl::DeleteShader(*id);
            }
        }

        self.id = shader_program_id;

        Ok(())
    }

    pub fn compile_shader(code: &str, type_: ShaderCompileType) -> Result<u32, GlError> {
        // let mut shader_file = File::open(path)?;
        // let mut shader_code = String::new();

        // println!("DEBUG::SHADER::{}::READING_FILE: {}", type_, path);

        // shader_file.read_to_string(&mut shader_code)?;

        let shader_code = CString::new(code.as_bytes())?;
        let shader_type = match type_ {
            ShaderCompileType::Vertex => gl::VERTEX_SHADER,
            ShaderCompileType::Geometry => gl::GEOMETRY_SHADER,
            ShaderCompileType::Fragment => gl::FRAGMENT_SHADER,
            _ => gl::VERTEX_SHADER // Default to vertex shader just in case
        };
        let shader;

        unsafe {
            shader = gl::CreateShader(shader_type);

            gl::ShaderSource(shader, 1, &shader_code.as_ptr(), ptr::null());

            println!("DEBUG::SHADER::{}::COMPILING_SHADER", type_);

            gl::CompileShader(shader);
            ShaderProgram::check_compile_errors(shader, type_.clone())?;
        }

        println!("DEBUG::SHADER::{}::COMPILATION_COMPLETE", type_);

        Ok(shader)
    }

    pub unsafe fn check_compile_errors(id: u32, type_: ShaderCompileType) -> Result<(), GlError> {
        let mut success = gl::FALSE as gl::types::GLint;
        match type_ {
            ShaderCompileType::Program => gl::GetProgramiv(id, gl::LINK_STATUS, &mut success),
            _ => gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success)
        }

        let error;

        if success != gl::TRUE as gl::types::GLint {
            let mut len: gl::types::GLint = 0;

            match type_ {
                ShaderCompileType::Program => {
                    gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
    
                    error = {
                        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                        buffer.extend([b' '].iter().cycle().take(len as usize));
                        CString::from_vec_unchecked(buffer)
                    };
        
                    gl::GetProgramInfoLog(
                        id,
                        len,
                        ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar
                    );
                },
                _ => {
                    gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    
                    error = {
                        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                        buffer.extend([b' '].iter().cycle().take(len as usize));
                        CString::from_vec_unchecked(buffer)
                    };
        
                    gl::GetShaderInfoLog(
                        id,
                        len,
                        ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar
                    );
                }
            }

            // into_string() shouldn't error since it's directly from OpenGL
            return Err(GlError::ShaderCompileError(type_, id, error.into_string().unwrap()))
        }

        Ok(())
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) } // Cannot error as program always exists by this point
    }

    // Safe functions check whetehr the uniform exists, and passes an error
    pub unsafe fn set_uniform<F: Fn(i32)>(&self, name: &str, uniform_func: F) -> Result<(), GlError> {
        let cstr = CString::new(name)?;
        let location = gl::GetUniformLocation(self.id, cstr.as_ptr());

        if location == -1 {
            return Err(GlError::UniformNotFound(name.to_owned(), self.id));
        }

        uniform_func(location);

        Ok(())
    }

    pub fn set_bool(&self, name: &str, value: bool) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::Uniform1i(location, value as gl::types::GLint))
        }
    }

    pub fn set_int(&self, name: &str, value: i32) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::Uniform1i(location, value as gl::types::GLint))
        }
    }

    pub fn set_float(&self, name: &str, value: f32) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::Uniform1f(location, value as gl::types::GLfloat))
        }
    }

    pub fn set_vector_3(&self, name: &str, value: &Vector3<f32>) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::Uniform3fv(location, 1, value.as_ptr()))
        }
    }

    pub fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::Uniform3f(location, x, y, z))
        }
    }

    pub fn set_vector_4(&self, name: &str, value: &Vector4<f32>) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::Uniform4fv(location, 1, value.as_ptr()))
        }
    }

    pub fn set_vec4(&self, name: &str, w: f32, x: f32, y: f32, z: f32) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::Uniform4f(location, w, x, y, z))
        }
    }

    pub fn set_mat4(&self, name: &str, value: &Matrix4<f32>) -> Result<(), GlError> {
        unsafe {
            self.set_uniform(name, |location| gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr()))
        }
    }

    // Unsafe versions do not check whether the uniforms are present
    pub unsafe fn set_uniform_unsafe<F: Fn(i32)>(&self, name: &str, uniform_func: F) -> Result<(), GlError> {
        let cstr = CString::new(name)?;
        let location = gl::GetUniformLocation(self.id, cstr.as_ptr());

        uniform_func(location);

        Ok(())
    }

    pub unsafe fn set_bool_unsafe(&self, name: &str, value: bool) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::Uniform1i(location, value as gl::types::GLint))
    }

    pub unsafe fn set_int_unsafe(&self, name: &str, value: i32) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::Uniform1i(location, value as gl::types::GLint))
    }

    pub unsafe fn set_float_unsafe(&self, name: &str, value: f32) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::Uniform1f(location, value as gl::types::GLfloat))
    }

    pub unsafe fn set_vector_3_unsafe(&self, name: &str, value: &Vector3<f32>) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::Uniform3fv(location, 1, value.as_ptr()))
    }

    pub unsafe fn set_vec3_unsafe(&self, name: &str, x: f32, y: f32, z: f32) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::Uniform3f(location, x, y, z))
    }

    pub unsafe fn set_vector_4_unsafe(&self, name: &str, value: &Vector4<f32>) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::Uniform4fv(location, 1, value.as_ptr()))
    }

    pub unsafe fn set_vec4_unsafe(&self, name: &str, w: f32, x: f32, y: f32, z: f32) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::Uniform4f(location, w, x, y, z))
    }

    pub unsafe fn set_mat4_unsafe(&self, name: &str, value: &Matrix4<f32>) -> Result<(), GlError> {
        self.set_uniform_unsafe(name, |location| gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr()))
    }

    pub fn bind_to_ubo(&self, name: &str) -> Result<(), GlError> {
        let cstr = CString::new(name)?;

        unsafe {
            let uniform_block_index = gl::GetUniformBlockIndex(self.id, cstr.as_ptr());
            
            if uniform_block_index == gl::INVALID_INDEX {
                return Err(GlError::UniformInvalidIndex(name.to_owned(), self.id));
            }

            gl::UniformBlockBinding(self.id, uniform_block_index, 0);
        }

        Ok(())
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::UseProgram(0);
            gl::DeleteProgram(self.id);
        }
    }
}

#[derive(Debug, Clone)]
pub enum ShaderCompileType {
    Program,
    Vertex,
    Fragment,
    Geometry
}

impl Display for ShaderCompileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ShaderCompileType::Program => "Program",
            ShaderCompileType::Vertex => "Vertex",
            ShaderCompileType::Geometry => "Geometry",
            ShaderCompileType::Fragment => "Fragment"
        };

        write!(f, "{}", str)
    }
}

#[derive(Default)]
pub struct ShaderCodeBundle {
    pub vertex: Option<String>,
    pub geometry: Option<String>,
    pub fragment: Option<String>
}

impl ShaderCodeBundle {
    pub fn get_vec(&self) -> Vec<(&Option<String>, ShaderCompileType)> {
        vec![
            (&self.vertex, ShaderCompileType::Vertex),
            (&self.geometry, ShaderCompileType::Geometry),
            (&self.fragment, ShaderCompileType::Fragment)
        ]
    }
}