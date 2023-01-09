pub struct GlImage {
    pub bytes: Vec<u8>,
    pub internal_format: gl::types::GLenum,
    pub data_format: gl::types::GLenum,
    pub width: i32,
    pub height: i32
}