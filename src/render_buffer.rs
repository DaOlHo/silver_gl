pub struct RenderBuffer {
    id: u32
}

impl RenderBuffer {
    // Requires framebuffer to be bound
    pub fn new(width: i32, height: i32) -> RenderBuffer {
        let mut renderbuffer = RenderBuffer {
            id: 0
        };

        unsafe {
            // Create renderbuffer
            gl::CreateRenderbuffers(1, &mut renderbuffer.id);
            gl::NamedRenderbufferStorage(
                renderbuffer.id,
                gl::DEPTH24_STENCIL8,
                width,
                height
            );
        }

        renderbuffer
    }

    pub unsafe fn resize(&self, width: i32, height: i32) {
        gl::NamedRenderbufferStorage(
            self.id,
            gl::DEPTH24_STENCIL8,
            width,
            height
        );
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.id);
        }
    }
}