mod shader_program;
mod camera;
mod view_3d_render_pipeline;
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
mod view_3d_scene;
mod view_2d_render_pipeline;
mod gl_image;

pub use shader_program::*;
pub use camera::*;
pub use view_3d_render_pipeline::*;
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
pub use view_3d_scene::*;
pub use view_2d_render_pipeline::*;
pub use gl_image::*;

// TODO: Create resource manager that handles loading textures and VAO/VBOs, the gives reference counted pointers to the resources
// TODO:    Create game objects system that receives references to resources managed by the resource manager (engine specific thing?)
// TODO:    Game object per object in the engine, with the resource manager handling gameobjects -> transforms in model's tbo
// TODO: Move scenes and render pipelines out into engine
// TODO: Combine entire model into one draw call
// TODO:    Follow this: https://www.khronos.org/assets/uploads/developers/library/2013-siggraph-opengl-bof/Batch-and-Cull-in-OpenGL-BOF_SIGGRAPH-2013.pdf
// TODO:    This is more something you should follow once an engine is established
// TODO:    Don't need instanced rendering if everything is one draw call!
// TODO:    https://webglfundamentals.org/webgl/lessons/webgl-qna-drawing-many-different-models-in-a-single-draw-call.html
// TODO: Implement transparency (see if qsort is fast enough to do it each frame for each model of the scene?)
// TODO: Add simple and efficient lighting to everything (do serious research when it comes to doing this on forward and deffered pipelines)
// TODO: Implement multisampling on all render pipelines (maybe make multisampled versions of them?)
// TODO: Create test suite
// TODO: Finish LearnOpenGL and do all the guest articles as well
// TODO: Comments that use better-comments styles
// TODO: Create documentation using rust's documentation thing (Have GL errors section in docs as well as panics section)
// TODO: Maybe implement compute shaders to do matrix transforms once each frame but faster than on CPU?