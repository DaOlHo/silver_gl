use cgmath::{Matrix4, Vector3, Vector2};
use crate::Model;
use super::{Mesh, Vertex};

// TODO: Move to resource manageer
pub fn create_quad(model_transforms: Vec<Matrix4<f32>>) -> Model {
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

    let model = Model::new(
        vertices,
        indices,
        model_transforms, 
        vec![Mesh::new(0, 6)]
    );

    model
}