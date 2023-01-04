use std::rc::Rc;
use super::{GlError, GlImage, Framebuffer};

pub struct Texture {
    id: u32,
    target: gl::types::GLenum,
    pub path: String,
    can_resize: bool
}

impl Texture {
    pub fn from_file_2d(path: &str) -> Result<Texture, GlError> {
        let mut texture = Texture {
            id: 0,
            target: gl::TEXTURE_2D,
            path: path.to_owned(),
            can_resize: false
        };

        let gl_image = GlImage::from_file(path)?;
    
        unsafe {
            gl::CreateTextures(texture.target, 1, &mut texture.id);
            
            gl::TextureStorage2D(
                texture.id,
                1,
                gl_image.internal_format,
                gl_image.width,
                gl_image.height
            );

            gl::TextureSubImage2D(
                texture.id,
                0,
                0,
                0,
                gl_image.width,
                gl_image.height,
                gl_image.data_format,
                gl::UNSIGNED_BYTE,
                gl_image.bytes.as_ptr() as *const gl::types::GLvoid
            );
            
            gl::GenerateTextureMipmap(texture.id);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Ok(texture)
    }

    pub fn from_file_cubemap(faces: Vec<String>) -> Result<Texture, GlError> {
        if faces.len() != 6 {
            return Err(GlError::IncorrectSize(String::from("cubemap has less than 6 faces")));
        }

        let mut texture = Texture {
            id: 0,
            target: gl::TEXTURE_CUBE_MAP,
            path: faces[0].clone(), // Uses first texture as path
            can_resize: false
        };

        let mut face_imgs = Vec::new();

        for face in faces {
            face_imgs.push(GlImage::from_file(face.as_str())?);
        }

        unsafe {
            gl::CreateTextures(texture.target, 1, &mut texture.id);

            gl::TextureStorage2D(
                texture.id,
                1,
                face_imgs[0].internal_format,
                face_imgs[0].width,
                face_imgs[0].height
            );

            for (i, face) in face_imgs.iter().enumerate() {
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
    // Assumes framebuffer is bound
    pub fn for_framebuffer(framebuffer: &mut Framebuffer) -> (u32, Rc<Texture>) {
        let mut texture = Texture {
            id: 0,
            path: "".into(),
            target: gl::TEXTURE_2D,
            can_resize: true
        };

        // Get number of new texture
        let num: u32 = framebuffer.len() as u32;
        let (width, height) = framebuffer.get_size();

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

            // Bind to framebuffer
            gl::NamedFramebufferTexture(
                framebuffer.get_id(),
                gl::COLOR_ATTACHMENT0 + num,
                texture.id,
                0
            );
        }

        (gl::COLOR_ATTACHMENT0 + num, Rc::new(texture))
    }

    pub fn ready_texture(&self, num: u32) {
        unsafe {
            gl::BindTextureUnit(num, self.id);
        }
    }

    pub fn resize(&self, width: i32, height: i32) -> Result<(), GlError> {
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
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}