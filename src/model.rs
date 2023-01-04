use std::{path::Path, rc::Rc};
use cgmath::{vec2, vec3, Matrix4, Vector3, Zero};
use memoffset::offset_of;
use crate::Buffer;

use super::{ShaderProgram, Mesh, Vertex, Texture, GlError, VertexArray};

// TODO: Use multidraw to have one VAO and transform buffer per model, and therefore one draw call
pub struct Model {
    pub meshes: Vec<Mesh>,
    // Stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    // Reference counter to ensure textures are dropped properly.
    pub textures_loaded: Vec<Rc<Texture>>, // TODO: Will be moved out to resource manager
    pub directory: String,
    pub obj_path: String,
    pub vao: VertexArray,
    pub vbo: Buffer<Vertex>,
    pub ebo: Buffer<u32>,
    pub tbo: Buffer<Matrix4<f32>>
}

impl Model {
    pub fn new(path: &str, model_transforms: Vec<Matrix4<f32>>) -> Result<Model, GlError> {
        let mut model = Model {
            meshes: Vec::new(),
            textures_loaded: Vec::new(),
            directory: String::new(),
            obj_path: String::new(),
            vao: VertexArray::new(),
            vbo: Buffer::new(),
            ebo: Buffer::new(),
            tbo: Buffer::new()
        };

        let (mut vertices, mut indices) = model.load_model(path)?;
        model.calc_vertex_tangents(&mut vertices, &mut indices);
        model.setup_model(vertices, indices);
        model.setup_transform_attribute(model_transforms);

        Ok(model)
    }

    pub fn from_raw(mut vertices: Vec<Vertex>, mut indices: Vec<u32>, model_transforms: Vec<Matrix4<f32>>) -> Model {
        let mut model = Model {
            meshes: Vec::new(),
            textures_loaded: Vec::new(),
            directory: String::new(),
            obj_path: String::new(),
            vao: VertexArray::new(),
            vbo: Buffer::new(),
            ebo: Buffer::new(),
            tbo: Buffer::new()
        };

        model.calc_vertex_tangents(&mut vertices, &mut indices);
        model.setup_model(vertices, indices);
        model.setup_transform_attribute(model_transforms);

        model
    }

    pub fn draw(&self, shader_program: &ShaderProgram) -> Result<(), GlError> {
        unsafe {
            self.vao.bind();

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

    pub fn load_model(&mut self, path: &str) -> Result<(Vec<Vertex>, Vec<u32>), GlError> {
        let path = Path::new(path);
        self.obj_path = path.to_str().unwrap().to_owned();
        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        
        let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);

        let (models, materials) = obj?;
        let materials = materials?;

        // Combine all meshes for optimized rendering
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // Push to model vertices
            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(
                    Vertex {
                        position: vec3(p[i*3], p[i*3+1], p[i*3+2]),
                        normal: vec3(n[i*3], n[i*3+1], n[i*3+2]),
                        tex_coord: vec2(t[i*2], t[i*2+1]),
                        ..Vertex::default()
                    }
                )
            }

            // Push to model indices while adjusting for offset
            let offset = indices.len();
            let mut adjusted_indices: Vec<u32> = mesh.indices.iter().map(|index| { index + offset as u32 }).collect();
            indices.append(&mut adjusted_indices);

            // Process material
            let mut gl_mesh = Mesh::new(offset, mesh.indices.len() as i32);
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // Diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture = self.load_material_texture(&material.diffuse_texture)?;
                    gl_mesh.diffuse_textures.push(texture);
                } else {
                    gl_mesh.diffuse = vec3(material.diffuse[0], material.diffuse[1], material.diffuse[2]);
                }
                // Specular map
                if !material.specular_texture.is_empty() {
                    let texture = self.load_material_texture(&material.specular_texture)?;
                    gl_mesh.specular_textures.push(texture);
                } else {
                    gl_mesh.specular = vec3(material.specular[0], material.specular[1], material.specular[2]);
                }
                // Normal map
                if !material.normal_texture.is_empty() {
                    let texture = self.load_material_texture(&material.normal_texture)?;
                    gl_mesh.normal_textures.push(texture);
                }
                // Shininess map
                if !material.shininess_texture.is_empty() {
                    let texture = self.load_material_texture(&material.shininess_texture)?;
                    gl_mesh.shininess_textures.push(texture);
                } else {
                    gl_mesh.shininess = material.shininess; // Get all-mesh shininess if there is no map present
                }
            }

            self.meshes.push(gl_mesh);
        }

        Ok((vertices, indices))
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

    pub fn calc_vertex_tangents(&mut self, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
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

    pub fn load_material_texture(&mut self, path: &str) -> Result<Rc<Texture>, GlError> {
        let texture = self.textures_loaded.iter().find(|t| t.path == path);
        if let Some(texture) = texture {
            return Ok(Rc::clone(texture));
        }

        let path = format!("{}/{}", &self.directory, path);
        let texture = Rc::new(Texture::from_file_2d(&path)?);
        let result = Rc::clone(&texture);

        // Send owned RC to loaded textures, and reference to the actual mesh
        self.textures_loaded.push(texture);
        Ok(result)
    }
}