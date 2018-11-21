extern crate gl;

use std;
use gl::types::{ GLuint, GLsizeiptr, GLvoid };

pub struct ArrayBuffer {
    vbo: GLuint,
}

/// Abstraction over opengl buffer objects
impl ArrayBuffer {
    pub fn new() -> ArrayBuffer {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        return ArrayBuffer{ vbo };
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn set_data_static_draw<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<T>()) as GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
        }
    }

}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}
