use cgmath::{vec2, Matrix4, Vector3, Zero};
use memoffset::offset_of;
use crate::Buffer;
use super::{ShaderProgram, Mesh, Vertex, GlError, VertexArray, gl};

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub vao: VertexArray,
    pub vbo: Buffer<Vertex>,
    pub ebo: Buffer<u32>,
    pub tbo: Buffer<Matrix4<f32>>
}

impl Model {
    pub fn new(
        mut vertices: Vec<Vertex>,
        mut indices: Vec<u32>,
        model_transforms: Vec<Matrix4<f32>>,
        meshes: Vec<Mesh>
    ) -> Model {
        let mut model = Model {
            meshes,
            vao: VertexArray::new(),
            vbo: Buffer::new(),
            ebo: Buffer::new(),
            tbo: Buffer::new()
        };

        Model::calc_vertex_tangents(&mut vertices, &mut indices);
        model.setup_model(vertices, indices);
        model.setup_transform_attribute(model_transforms);

        model
    }

    pub fn draw(&self, shader_program: &ShaderProgram) -> Result<(), GlError> {
        unsafe {
            self.vao.bind();

            // TODO: work on making this work with textures so there is one draw call
            for mesh in &self.meshes {
                mesh.set_textures(shader_program)?;
                self.vao.draw_elements_offset(
                    mesh.get_count(),
                    mesh.get_offset(),
                    self.tbo.len() as i32
                );
    
                // Set back to defaults once configured
                gl::ActiveTexture(gl::TEXTURE0);
            }

            gl::BindVertexArray(0);
        }

        Ok(())
    }

    pub fn setup_model(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>) {
        self.vao.add_vertex_buffer(&mut self.vbo);
        self.vao.set_element_buffer(&mut self.ebo);

        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, position) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, normal) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 2, offset_of!(Vertex, tex_coord) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, tangent) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, bitangent) as u32, gl::FLOAT);

        self.vbo.set_data(vertices);
        self.ebo.set_data(indices);
    }
    
    pub fn setup_transform_attribute(&mut self, model_transforms: Vec<Matrix4<f32>>) {
        self.vao.add_vertex_buffer(&mut self.tbo);
        self.vao.add_attrib_divisor(&mut self.tbo, 4);
        self.tbo.set_data_mut(model_transforms);
    }

    pub fn calc_vertex_tangents(vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        for i in 0..(indices.len() / 3) {
            let index = i * 3;

            let index1 = indices[index] as usize;
            let index2 = indices[index + 1] as usize;
            let index3 = indices[index + 2] as usize;

            // Get positions for the vertices that make up the triangle
            let pos1 = vertices[index1].position;
            let pos2 = vertices[index2].position;
            let pos3 = vertices[index3].position;

            // Get corresponding texture coordinates
            let uv1 = vertices[index1].tex_coord;
            let uv2 = vertices[index2].tex_coord;
            let uv3 = vertices[index3].tex_coord;

            // Calculate deltas
            let edge1 = pos2 - pos1;
            let edge2 = pos3 - pos1;
            let mut delta_uv1 = uv2 - uv1;
            let mut delta_uv2 = uv3 - uv1;

            // Slight correction for angles to be more accurate
            let dir_correction: bool = (delta_uv2.x * delta_uv1.y - delta_uv2.y * delta_uv1.x) < 0.0;
            let dir_correction: f32 = if dir_correction { -1.0 } else { 1.0 };

            if delta_uv1.x * delta_uv2.y == delta_uv1.y * delta_uv2.x {
                delta_uv1 = vec2(0.0, 1.0);
                delta_uv2 = vec2(1.0, 0.0);
            }

            // Create tangent and bitangent vectors
            let mut tangent: Vector3<f32> = Vector3::zero();
            let mut bitangent: Vector3<f32> = Vector3::zero();

            // Calculate tangent vector
            tangent.x = dir_correction * (edge2.x * delta_uv1.y - edge1.x * delta_uv2.y);
            tangent.y = dir_correction * (edge2.y * delta_uv1.y - edge1.y * delta_uv2.y);
            tangent.z = dir_correction * (edge2.z * delta_uv1.y - edge1.z * delta_uv2.y);
            
            // Calculate bitangent vector
            bitangent.x = dir_correction * ( - edge2.x * delta_uv1.x + edge1.x * delta_uv2.x);
            bitangent.y = dir_correction * ( - edge2.y * delta_uv1.x + edge1.y * delta_uv2.x);
            bitangent.z = dir_correction * ( - edge2.z * delta_uv1.x + edge1.z * delta_uv2.x);

            // Set tangent vector to all vertices of the triangle
            vertices[index1].tangent = tangent;
            vertices[index2].tangent = tangent;
            vertices[index3].tangent = tangent;            

            vertices[index1].bitangent = bitangent;
            vertices[index2].bitangent = bitangent;
            vertices[index3].bitangent = bitangent;
        }
    }
}