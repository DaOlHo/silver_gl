use std::rc::Rc;
use cgmath::{Matrix4, Vector3, Vector2};
use super::{Texture, Mesh, Vertex, GlError};

pub fn create_quad(model_transforms: Vec<Matrix4<f32>>) -> Mesh {
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

    let mesh = Mesh::new(
        vertices,
        indices,
        model_transforms
    );

    mesh
}

pub fn create_quad_from_paths(
    diff_map: Option<&str>,
    norm_map: Option<&str>,
    disp_map: Option<&str>,
    model_transforms: Vec<Matrix4<f32>>
) -> Result<Mesh, GlError> {    
    let mut quad = create_quad(model_transforms);

    if let Some(diff_path) = diff_map {
        quad.diffuse_textures.push(
            Rc::new(Texture::from_file_2d(diff_path)?)
        );
    }
    if let Some(norm_path) = norm_map {
        quad.normal_textures.push(
            Rc::new(Texture::from_file_2d(norm_path)?)
        )
    };
    if let Some(disp_path) = disp_map {
        quad.displacement_textures.push(
            Rc::new(Texture::from_file_2d(disp_path)?)
        );
    }

    Ok(quad)
}