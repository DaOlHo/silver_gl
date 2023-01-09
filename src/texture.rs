use super::{GlError, GlImage};

pub struct Texture {
    id: u32,
    target: gl::types::GLenum,
    can_resize: bool
}

impl Texture {
    pub fn from_2D(image: GlImage) -> Texture {
        let mut texture = Texture {
            id: 0,
            target: gl::TEXTURE_2D,
            can_resize: false
        };
    
        unsafe {
            gl::CreateTextures(texture.target, 1, &mut texture.id);
            
            gl::TextureStorage2D(
                texture.id,
                1,
                image.internal_format,
                image.width,
                image.height
            );

            gl::TextureSubImage2D(
                texture.id,
                0,
                0,
                0,
                image.width,
                image.height,
                image.data_format,
                gl::UNSIGNED_BYTE,
                image.bytes.as_ptr() as *const gl::types::GLvoid
            );
            
            gl::GenerateTextureMipmap(texture.id);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        texture
    }

    pub fn from_cubemap(faces: Vec<GlImage>) -> Result<Texture, GlError> {
        if faces.len() != 6 {
            return Err(GlError::IncorrectSize(String::from("cubemap has less than 6 faces")));
        }

        let mut texture = Texture {
            id: 0,
            target: gl::TEXTURE_CUBE_MAP,
            can_resize: false
        };

        unsafe {
            gl::CreateTextures(texture.target, 1, &mut texture.id);

            gl::TextureStorage2D(
                texture.id,
                1,
                faces[0].internal_format,
                faces[0].width,
                faces[0].height
            );

            for (i, face) in faces.iter().enumerate() {
                // Texture::load_file(face, gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32)?;
                gl::TextureSubImage3D(
                    texture.id,
                    0,
                    0,
                    0,
                    i as i32,
                    face.width,
                    face.height,
                    1,
                    face.data_format,
                    gl::UNSIGNED_BYTE,
                    face.bytes.as_ptr() as *const gl::types::GLvoid
                );
            }

            gl::TextureParameteri(texture.id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }

        Ok(texture)
    }

    // Doesn't need GlError since this only generates gl callback errors
    pub fn new_mut(width: i32, height: i32) -> Texture {
        let mut texture = Texture {
            id: 0,
            target: gl::TEXTURE_2D,
            can_resize: true
        };

        unsafe {
            // Create empty texture
            // Does not use DFA so that the texture can be resized
            gl::CreateTextures(texture.target, 1, &mut texture.id);
            gl::BindTexture(texture.target, texture.id);
            gl::TexImage2D(
                texture.target,
                0,
                gl::RGBA16F as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null()
            );
            gl::BindTexture(texture.target, 0);

            // Nearest just for simplicity
            gl::TextureParameteri(texture.id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        texture
    }

    pub fn ready_texture(&self, num: u32) {
        unsafe {
            gl::BindTextureUnit(num, self.id);
        }
    }

    // Unsafe because it doesn't need to be marked as mutable, which would interfere with RC
    // TODO: CHANGE WHEN WRITING RESOURCE MANAGER!
    pub unsafe fn resize(&self, width: i32, height: i32) -> Result<(), GlError> {
        if !self.can_resize {
            return Err(GlError::CannotResize(self.id));
        }

        unsafe {
            gl::BindTexture(self.target, self.id);
            // Resizes texture on same ID
            gl::TexImage2D(
                self.target,
                0,
                gl::RGBA16F as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null()
            );
            gl::BindTexture(self.target, 0);
        }

        Ok(())
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}