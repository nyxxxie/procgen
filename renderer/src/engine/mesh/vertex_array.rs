extern crate gl;

use gl::types::{ GLuint, };

/// Abstraction over opengl vertex array objects
pub struct VertexArray {
    vao: GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        return VertexArray{ vao };
    }
    
    pub fn bind(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindVertexArray(0);
		}
	}
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}


