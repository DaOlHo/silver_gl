use super::gl;

pub struct GlImage {
    pub bytes: Vec<u8>,
    pub internal_format: gl::types::GLenum,
    pub data_format: gl::types::GLenum,
    pub width: i32,
    pub height: i32
}

impl GlImage {
    pub fn sub_image(
        &self,
        offset_x: i32,
        offset_y: i32,
        width: i32,
        height: i32
    ) -> Vec<u8> {
        let pixel_size = match self.data_format {
            gl::RED => 1,
            gl::RG => 2,
            gl::RGB => 3,
            gl::RGBA => 4,
            _ => panic!() // Should not be possible
        };

        let mut result = Vec::new();

        for i in 0..height {
            let start = ((self.width * pixel_size * offset_y) + (offset_x * pixel_size) + (i * self.width * pixel_size)) as usize;
            let stride = (width * pixel_size) as usize;

            let bytes = self.bytes.get(start..(start + stride)).unwrap();

            result.extend_from_slice(bytes);
        }

        result
    }
}