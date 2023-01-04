use super::Framebuffer;

pub struct RenderBuffer {
    id: u32
}

impl RenderBuffer {
    // Requires framebuffer to be bound
    pub fn for_framebuffer(framebuffer: &mut Framebuffer) -> RenderBuffer {
        let mut renderbuffer = RenderBuffer {
            id: 0
        };

        let (width, height) = framebuffer.get_size();

        unsafe {
            // Create renderbuffer
            gl::CreateRenderbuffers(1, &mut renderbuffer.id);
            gl::NamedRenderbufferStorage(
                renderbuffer.id,
                gl::DEPTH24_STENCIL8,
                width,
                height
            );

            // Bind to FB
            gl::NamedFramebufferRenderbuffer(
                framebuffer.get_id(),
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                renderbuffer.id
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
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.id);
        }
    }
}