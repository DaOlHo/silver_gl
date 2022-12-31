mod shader_program;
mod camera;
mod view_3d_render_pipeline;
mod mesh;
mod model;
mod quad;
mod skybox;
mod uniform_buffer;
mod error;
mod vert_array;
mod buffer_obj;
mod texture;
mod vertex;
mod framebuffer;
mod render_buffer;
mod render_pipeline;
mod scene;
mod view_3d_scene;
mod view_2d_render_pipeline;

pub use shader_program::*;
pub use camera::*;
pub use view_3d_render_pipeline::*;
pub use mesh::*;
pub use model::*;
pub use quad::*;
pub use skybox::*;
pub use uniform_buffer::*;
pub use error::*;
pub use vert_array::*;
pub use buffer_obj::*;
pub use texture::*;
pub use vertex::*;
pub use framebuffer::*;
pub use render_buffer::*;
pub use render_pipeline::*;
pub use scene::*;
pub use view_3d_scene::*;
pub use view_2d_render_pipeline::*;

// TODO: IMMEDIATE NEXT STEPS:
// TODO:    Create transform and position system
// TODO:        Support both instanced and non instanced objects
// TODO:            Both contain links to model that draws instanced objects and has the buffer data transforms (see if this has a performance impact for non-instanced draw objects)
// TODO:            Instanced draw objects link to model and essentially just hold transormation info and index that is then updated in the model, and then the model needs to draw
// TODO:            Non-instanced draw objects link to model and have their own draw function, overwriting the buffer data transforms with one transform and drawing (this would need to be set every draw)
// TODO:            This system should allow for one model and one shader system, while allowing for efficient instanced rendering and actual individual rendered objects, which allows transparency
// TODO:            For this, keep a table of loaded models to add references, and create objects directly from model path
// TODO:        Update transform function that takes the object's position and rotation and makes it a transform matrix
// TODO: Used the gl::Named pattern in the rest of the lib, not just VAO and buffers
// TODO: Implement transparency (see if qsort is fast enough to do it each frame for each model of the scene?)
// TODO: Add simple and efficient lighting to everything (do serious research when it comes to doing this on forward and deffered pipelines)
// TODO: Implement multisampling on all render pipelines (maybe make multisampled versions of them?)
// TODO: Create test suite
// TODO: Comments that use better-comments styles
// TODO: Create documentation using rust's documentation thing (Have GL errors section in docs as well as panics section)