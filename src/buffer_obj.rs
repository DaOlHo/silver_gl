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

    pub fn send_data(&self) {
        unsafe {
            gl::NamedBufferStorage(
                self.id,
                (self.data.len() * std::mem::size_of::<T>()) as isize,
                self.data.as_ptr() as *const gl::types::GLvoid,
                0 as gl::types::GLbitfield
            );
        }
    }

    pub fn send_data_mut(&self) {
        unsafe {
            gl::NamedBufferData(
                self.id,
                (self.data.len() * std::mem::size_of::<T>()) as isize,
                self.data.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW
            );
        }
    }

    pub unsafe fn send_data_index(&self, index: usize) {
        let size = std::mem::size_of::<T>();

        unsafe {
            gl::NamedBufferSubData(
                self.id,
                (index * size) as isize,
                size as isize,
                self.data.as_ptr().add(index) as *const gl::types::GLvoid
            );
        }
    }

    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
        self.send_data();
    }

    pub fn set_data_mut(&mut self, data: Vec<T>) {
        self.data = data;
        self.send_data_mut();
    }

    // Push and remove are expensive because resizing needs to occur.
    // They also make the data mutable, so keep that in mind
    pub fn push(&mut self, data: T) {
        self.data.push(data);
        self.send_data_mut();
    }
    
    // Panics if index out of bounds
    pub fn remove(&mut self, index: usize) {
        self.data.remove(index);
        self.send_data_mut();
    }

    // These methods are unsafe because they modify the inner data without sending,
    // which is intended to be used to batch changes before sending it all at once
    pub unsafe fn push_to_inner(&mut self, data: T) {
        self.data.push(data);
    }

    pub unsafe fn clear_inner(&mut self) {
        self.data.clear();
    }

    // Cheaper since there is no resize, but requires mutability.
    // Panics if out of bounds
    pub fn set_data_index(&mut self, data: T, index: usize) {
        self.data[index] = data;
        unsafe { self.send_data_index(index) };
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    // Unsafe functions designed to be called from VAO
    // These need to exist because the buffer handles its own binding_index
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

    pub unsafe fn bind_to_vao_attrib(&mut self, vao_id: u32, attrib_index: u32) {
        gl::VertexArrayAttribBinding(vao_id, attrib_index, self.binding_index)
    }

    pub unsafe fn set_divisor(&mut self, vao_id: u32, divisor: u32) {
        gl::VertexArrayBindingDivisor(vao_id, self.binding_index, divisor);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}