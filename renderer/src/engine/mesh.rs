// TODO:  create a Vertex object where we can set position, color,  texture
// coords, etc.  These will automaticaly be linked to attributes.
// Maybe expose the first raw interface and then put this on top?
//
// Also, create a premade mesh generator (colored cube, textured cube, cylinder, quad, etc)


/// Represents a mesh of vertices.
pub struct Mesh {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint
}
