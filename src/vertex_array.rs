use super::{Buffer, gl};

pub struct VertexArray {
    id: u32,
    attrib_index: u32,
    buffer_index: u32
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vert_array = VertexArray {
            id: 0, attrib_index: 0, buffer_index: 0
        };

        unsafe {
            gl::CreateVertexArrays(1, &mut vert_array.id);
        }

        vert_array
    }

    pub fn add_vertex_buffer<T>(&mut self, buffer: &mut Buffer<T>) {
        unsafe {
            buffer.add_vertex_to_vertex_array(self.id, self.buffer_index);
        }

        self.buffer_index += 1;
    }

    pub fn set_element_buffer<T>(&mut self, buffer: &mut Buffer<T>) {
        unsafe {
            gl::VertexArrayElementBuffer(self.id, buffer.get_id());
        }
    }

    pub fn add_attrib<T>(&mut self, buffer: &mut Buffer<T>, size: i32, offset: u32, type_: gl::types::GLenum) {
        unsafe {
            gl::EnableVertexArrayAttrib(self.id, self.attrib_index);
            gl::VertexArrayAttribFormat(
                self.id,
                self.attrib_index,
                size,
                type_,
                gl::FALSE,
                offset
            );

            buffer.bind_to_vao_attrib(self.id, self.attrib_index);
        }

        self.attrib_index += 1;
    }

    // For adding things like mat4 (types that are larger than 4*f32s but are multiples of it)
    pub fn add_attrib_divisor<T>(&mut self, buffer: &mut Buffer<T>, rows: i32) {
        // Row size is constant in OpenGL
        let size_vec4 = 16;

        for i in 0..rows {
            self.add_attrib(buffer, 4, (i * size_vec4) as u32, gl::FLOAT);
        }

        unsafe {
            buffer.set_divisor(self.id, 1);
        }
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    // Get count and instance_count from in-built buffer objects
    pub fn draw_elements(&self, count: i32, instance_count: i32) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                instance_count
            );
            gl::BindVertexArray(0);
        }
    }

    // Requires VAO to be bound already
    pub fn draw_elements_offset(&self, count: i32, offset: usize, instance_count: i32) {
        unsafe {
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                count,
                gl::UNSIGNED_INT,
                std::ptr::null::<u32>().add(offset) as *const gl::types::GLvoid,
                instance_count
            );
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}