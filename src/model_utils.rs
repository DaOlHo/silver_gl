use cgmath::{Matrix4, Vector3, Vector2, vec2, Zero};
use crate::{ModelTrait, ModelCreateTrait};
use super::{Mesh, Vertex};

pub fn create_quad<T: ModelTrait + ModelCreateTrait>(model_transforms: Vec<Matrix4<f32>>) -> T {
    // Flat panel definition
    let vertices = vec![
        Vertex {
            position: Vector3::new(-1.0, 1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(0.0, 1.0),
            ..Vertex::default()
        },
        Vertex {
            position: Vector3::new(-1.0, -1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(0.0, 0.0),
            ..Vertex::default()
        },
        Vertex {
            position: Vector3::new(1.0, -1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(1.0, 0.0),
            ..Vertex::default()
        },
        Vertex {
            position: Vector3::new(1.0, 1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(1.0, 1.0),
            ..Vertex::default()
        }
    ];

    let indices = vec![
        0, 1, 2,
        0, 2, 3
    ];

    let model = T::new(
        vertices,
        indices,
        model_transforms, 
        vec![Mesh::new(0, 6)]
    );

    model
}

// Calculate lines perpendicular to normals for using normal maps
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