extern crate gl;

pub mod engine {
    use gl;

    /// Represents a shader program.
    ///
    /// Contains utility functions for common operations with shaders (making
    /// active, setting uniforms, etc).
    struct Shader {
        id: gl::types::GLuint,
    }

    impl Shader {
        /// Creates a Shader from a string containing GLSL source code.
        ///
        /// String must be a valid C string (no 0 bytes in the middle) as it will
        /// be converted and used as such.
        pub fn from_source(
            source: &str,
            shader_type: gl::types::GLenum
        ) -> Result<Shader, String> {
            let id = compile(source, shader_type)?;
            return Ok(Shader { id });
        }

        pub fn from_vert_source(source: &str) -> Result<Shader, String> {
            return Shader::from_source(source, gl::VERTEX_SHADER);
        }

        pub fn from_frag_source(source: &str) -> Result<Shader, String> {
            return Shader::from_source(source, gl::FRAGMENT_SHADER);
        }

        /// Compiles a shader from a string containing its source code.
        ///
        /// String must be a valid C string (no 0 bytes in the middle) as it will
        /// be converted and used as such.
        fn compile(
            source: &str,
            shader_type: gl::types::GLenum
        ) -> Result<gl::types::GLuint, String> {
            /* Convert the source string to a c_string.  */
            c_source = CString::new(source).unwrap();

            /* Create and compile shader */
            let id = unsafe { gl::CreateShader(kind); }
            unsafe {
                gl::ShaderSource(id, 1, &c_source.as_ptr(), std::ptr::null());
                gl::CompileShader(id);
            }

            /* Verify that our shader compiled */
            let mut success: gl::types::GLint = 1;
            gl::GetShaderiv(id, gl::INFO_LOG_STATUS, &mut success);

            if success == 0 {
                /* Create buffer to put error string into.  We start by allocating
                 * the buffer itself (first line), and then we fill it with space
                 * characters.  The second line basically fills the buffer with
                 * spaces so we don't confuse C.  The second line basically creates
                 * an iterator that will return a total of `len` spaces.  It
                 * creates an iterator array that contains a single space, makes
                 * the iterator return infinite spaces, and then limits it to
                 * output a maximim of len items.  */
                let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
                buffer.extend([b' '].iter().cycle().take(len));
                let error = CString::new(buffer).unwrap();

                /* Copy error into our string container */
                unsafe {
                    
                }

                return Err(error.to_string_lossy().into_owned());
            }

            return Ok(id);
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }

    /// Represents a shader program.
    ///
    /// Contains utility functions for common operations with shaders (making
    /// active, setting uniforms, etc).
    struct Shader {
        id: gl::types::GLuint,
    }

    impl Shader {
        /// Creates a Shader from a string containing GLSL source code.
        ///
        /// String must be a valid C string (no 0 bytes in the middle) as it will
        /// be converted and used as such.
        pub fn from_source(
            source: &str,
            shader_type: gl::types::GLenum
        ) -> Result<Shader, String> {
            let id = compile(source, shader_type)?;
            return Ok(Shader { id });
        }

        pub fn from_vert_source(source: &str) -> Result<Shader, String> {
            return Shader::from_source(source, gl::VERTEX_SHADER);
        }

        pub fn from_frag_source(source: &str) -> Result<Shader, String> {
            return Shader::from_source(source, gl::FRAGMENT_SHADER);
        }

        /// Compiles a shader from a string containing its source code.
        ///
        /// String must be a valid C string (no 0 bytes in the middle) as it will
        /// be converted and used as such.
        fn compile(
            source: &str,
            shader_type: gl::types::GLenum
        ) -> Result<gl::types::GLuint, String> {
            /* Convert the source string to a c_string.  */
            c_source = CString::new(source).unwrap();

            /* Create and compile shader */
            let id = unsafe { gl::CreateShader(kind); }
            unsafe {
                gl::ShaderSource(id, 1, &c_source.as_ptr(), std::ptr::null());
                gl::CompileShader(id);
            }

            /* Verify that our shader compiled */
            let mut success: gl::types::GLint = 1;
            gl::GetShaderiv(id, gl::INFO_LOG_STATUS, &mut success);

            if success == 0 {
                /* Create buffer to put error string into.  We start by allocating
                 * the buffer itself (first line), and then we fill it with space
                 * characters.  The second line basically fills the buffer with
                 * spaces so we don't confuse C.  The second line basically creates
                 * an iterator that will return a total of `len` spaces.  It
                 * creates an iterator array that contains a single space, makes
                 * the iterator return infinite spaces, and then limits it to
                 * output a maximim of len items.  */
                let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
                buffer.extend([b' '].iter().cycle().take(len));
                let error = CString::new(buffer).unwrap();

                /* Copy error into our string container */
                unsafe {
                    
                }

                return Err(error.to_string_lossy().into_owned());
            }

            return Ok(id);
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }
}
