mod shader_program;
mod mesh;
mod model;
mod quad;
mod skybox;
mod uniform_buffer;
mod error;
mod vertex_array;
mod buffer_obj;
mod texture;
mod vertex;
mod framebuffer;
mod render_buffer;
mod render_pipeline;
mod scene;
mod gl_image;

pub use shader_program::*;
pub use mesh::*;
pub use model::*;
pub use quad::*;
pub use skybox::*;
pub use uniform_buffer::*;
pub use error::*;
pub use vertex_array::*;
pub use buffer_obj::*;
pub use texture::*;
pub use vertex::*;
pub use framebuffer::*;
pub use render_buffer::*;
pub use render_pipeline::*;
pub use scene::*;
pub use gl_image::*;

// TODO: Implement transparency (see if qsort is fast enough to do it each frame for each model of the scene?)
// TODO: Add simple and efficient lighting to everything (do serious research when it comes to doing this on forward and deffered pipelines)
// TODO: Implement multisampling on all render pipelines (maybe make multisampled versions of them?)
// TODO: Create test suite
// TODO: Finish LearnOpenGL and do all the guest articles as well
// TODO: Comments that use better-comments styles
// TODO: Create documentation using rust's documentation thing (Have GL errors section in docs as well as panics section)
// TODO: Maybe implement compute shaders to do matrix transforms once each frame but faster than on CPU?
