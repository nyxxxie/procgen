extern crate gl;

use std::mem::size_of;
use gl::types::{ GLuint, GLsizeiptr, GLvoid };

pub struct Buffer {
    vbo: GLuint,
	kind: GLuint,
}

/// Abstraction over opengl buffer objects
impl Buffer {
    pub fn new(kind: GLuint) -> Buffer {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        return Buffer{ vbo, kind };
    }

    pub fn new_array_buffer() -> Buffer {
		return Buffer::new(gl::ARRAY_BUFFER);
	}

    pub fn new_element_array_buffer() -> Buffer {
		return Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
	}

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.kind, 0);
        }
    }

	pub fn set_data<T>(&self, data_type: GLuint, data: &[T]) {
        unsafe {
            gl::BufferData(self.kind,
                (data.len() * size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid, data_type);
        }
	}

    pub fn set_data_static_draw<T>(&self, data: &[T]) {
		Buffer::set_data(self, gl::STATIC_DRAW, data);
    }

}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}
