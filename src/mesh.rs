use std::rc::Rc;

use cgmath::Vector3;
use super::{ShaderProgram, GlError, Texture};

// TODO: sort any meshes with alpha values and render them farthest to closest w/o depth buffer

pub struct Mesh {
    pub diffuse_textures: Vec<Rc<Texture>>,
    pub diffuse: Vector3<f32>,
    pub specular_textures: Vec<Rc<Texture>>,
    pub specular: Vector3<f32>,
    pub normal_textures: Vec<Rc<Texture>>,
    pub displacement_textures: Vec<Rc<Texture>>,
    pub shininess_textures: Vec<Rc<Texture>>,
    pub shininess: f32,
    buffer_offset: usize,
    buffer_count: i32
}

impl Mesh {
    pub fn new(buffer_offset: usize, buffer_count: i32) -> Mesh {
        Mesh {
            diffuse_textures: Vec::new(),
            diffuse: Vector3 { x: 0.0, y: 0.0, z: 0.0},
            specular_textures: Vec::new(),
            specular: Vector3 { x: 0.0, y: 0.0, z: 0.0},
            normal_textures: Vec::new(),
            displacement_textures: Vec::new(),
            shininess_textures: Vec::new(),
            shininess: 0.0,
            buffer_offset,
            buffer_count
        }
    }

    pub unsafe fn set_textures(&self, shader_program: &ShaderProgram) -> Result<(), GlError> {
        let mut i: i32 = 0;
        
        // Diffuse
        for texture in self.diffuse_textures.iter() {
            texture.ready_texture(i as u32);
            shader_program.set_int_unsafe(format!("material.diffuse[{}]", i).as_str(), i)?;
            i += 1;
        }
        shader_program.set_int_unsafe("material.diffuseCount", self.diffuse_textures.len() as i32)?;
        if self.diffuse_textures.len() == 0 {
            shader_program.set_vector_3_unsafe("material.diffuseFloat", &self.diffuse)?;
        }

        // Specular
        for texture in self.specular_textures.iter() {
            texture.ready_texture(i as u32);
            shader_program.set_int_unsafe(format!("material.specular[{}]", i).as_str(), i)?;
            i += 1;
        }
        shader_program.set_int_unsafe("material.specularCount", self.specular_textures.len() as i32)?;
        if self.specular_textures.len() == 0 {
            shader_program.set_vector_3_unsafe("material.specularFloat", &self.specular)?;
        }

        // Normal
        for texture in self.normal_textures.iter() {
            texture.ready_texture(i as u32);
            shader_program.set_int_unsafe(format!("material.normal[{}]", i).as_str(), i)?;
            i += 1;
        }
        shader_program.set_int_unsafe("material.normalCount", self.normal_textures.len() as i32)?;

        // Displacement
        for texture in self.displacement_textures.iter() {
            texture.ready_texture(i as u32);
            shader_program.set_int_unsafe(format!("material.displacement[{}]", i).as_str(), i)?;
            i += 1;
        }
        shader_program.set_int_unsafe("material.displacementCount", self.displacement_textures.len() as i32)?;

        // Shininess
        for texture in self.shininess_textures.iter() {
            texture.ready_texture(i as u32);
            shader_program.set_int_unsafe(format!("material.shininess[{}]", i).as_str(), i)?;
            i += 1;
        }
        shader_program.set_int_unsafe("material.shininessCount", self.shininess_textures.len() as i32)?;
        if self.shininess_textures.len() == 0 {
            shader_program.set_float_unsafe("material.shininessFloat", self.shininess)?;
        }

        Ok(())
    }

    pub fn get_offset(&self) -> usize {
        self.buffer_offset
    }

    pub fn get_count(&self) -> i32 {
        self.buffer_count
    }
}