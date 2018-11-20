extern crate gl;

use std;
use std::ffi::{ CString };

/// Represents a shader.
pub struct Shader {
    id: gl::types::GLuint,
    kind: gl::types::GLenum
}
