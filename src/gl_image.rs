use super::GlError;
use image::DynamicImage::*;

pub struct GlImage {
    pub bytes: Vec<u8>,
    pub internal_format: gl::types::GLenum,
    pub data_format: gl::types::GLenum,
    pub width: i32,
    pub height: i32
}

impl GlImage {
    pub fn from_file(path: &str) -> Result<GlImage, GlError> {

        let img = image::io::Reader::open(path)?.decode()?;

        // TODO: if there is an alpha, mark texture as transparent
        let (internal_format, data_format) = match img {
            ImageLuma8(_) => (gl::RED, gl::RED),
            ImageLumaA8(_) => (gl::RG, gl::RG),
            ImageRgb8(_) => (gl::SRGB, gl::RGB),
            ImageRgba8(_) => (gl::SRGB_ALPHA, gl::RGBA),
            _ => (gl::SRGB, gl::RGB) // If nothing else, try default
        };

        Ok(
            GlImage {
                bytes: Vec::from(img.as_bytes()),
                internal_format,
                data_format,
                width: img.width() as i32,
                height: img.height() as i32
            }
        )
    }
}