pub struct Buffer<T> {
    id: u32,
    data: Vec<T>,
    binding_index: u32
}

impl<T> Buffer<T> {
    pub fn new() -> Buffer<T> {
        let mut buffer = Buffer {
            data: Vec::<T>::new(), id: 0, binding_index: 0
        };

        unsafe {
            gl::CreateBuffers(1, &mut buffer.id);
        }

        buffer
    }

    pub fn send_data(&mut self, data: Vec<T>) {
        self.data = data;

        unsafe {
            gl::NamedBufferStorage(
                self.id,
                (self.data.len() * std::mem::size_of::<T>()) as isize,
                self.data.as_ptr() as *const gl::types::GLvoid,
                0 as gl::types::GLbitfield
            )
        }
    }

    pub fn send_data_mut(&mut self, data: Vec<T>) {
        self.data = data;

        unsafe {
            gl::NamedBufferData(
                self.id,
                (self.data.len() * std::mem::size_of::<T>()) as isize,
                self.data.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW
            );
        }
    }

    // Unsafe functions designed to be called from VAO
    pub unsafe fn add_vertex_to_vertex_array(&mut self, vao_id: u32, binding_index: u32) {
        gl::VertexArrayVertexBuffer(
            vao_id,
            binding_index,
            self.id,
            0,
            std::mem::size_of::<T>() as gl::types::GLint
        );

        self.binding_index = binding_index;
    }

    pub unsafe fn add_element_to_vertex_array(&mut self, vao_id: u32) {
        gl::VertexArrayElementBuffer(vao_id, self.id);
    }

    pub unsafe fn bind_to_vao_attrib(&mut self, vao_id: u32, attrib_index: u32) {
        gl::VertexArrayAttribBinding(vao_id, attrib_index, self.binding_index)
    }

    pub unsafe fn set_divisor(&mut self, vao_id: u32, divisor: u32) {
        gl::VertexArrayBindingDivisor(vao_id, self.binding_index, divisor);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}