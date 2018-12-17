// TODO:  create a Vertex object where we can set position, color,  texture
// coords, etc.  These will automaticaly be linked to attributes.
// Maybe expose the first raw interface and then put this on top?
//
// Also, create a premade mesh generator (colored cube, textured cube, cylinder, quad, etc)
extern crate gl;
mod buffer;
mod vertex_array;

use std;
use std::mem::size_of;
use gl::types::{ GLuint, GLint, GLfloat, GLvoid };
use self::buffer::Buffer;
use self::vertex_array::VertexArray;

/// Represents a mesh of vertices.
pub struct Mesh {
    vao: VertexArray,
    uses_indices: bool,
	element_amt: usize,
}

impl Mesh {
	pub fn draw(&self, mode: GLuint) {
		self.vao.bind();
        if self.uses_indices {
		    unsafe {
                gl::DrawElements(
                    mode,
                    self.element_amt as GLint,
                    gl::UNSIGNED_INT,
                    std::ptr::null());
		    }
        } else {
		    unsafe {
                gl::DrawArrays(
                    mode,
                    0,  // starting index
                    self.element_amt as GLint);
		    }
        }
		self.vao.unbind();
	}

	pub fn draw_triangles(&self) {
		self.draw(gl::TRIANGLES);
	}

	pub fn draw_points(&self) {
		self.draw(gl::POINTS);
	}
}

struct Attribute {
	location: GLuint,
	component_amt: GLint
}

/// Stuff needed to build a mesh.
pub struct MeshBuilder {
	vertices: Vec<GLfloat>,
	indices: Vec<GLuint>,
	attributes: Vec<Attribute>,
}

impl MeshBuilder {
	pub fn new() -> MeshBuilder {
		return MeshBuilder{
			vertices: Vec::new(),
			indices: Vec::new(),
			attributes: Vec::new(),
		};
	}

				///* First triangle */
				//-0.5,  0.5, 0.0,  // upper left
				// 0.5,  0.5, 0.0,  // upper right
				//-0.5, -0.5, 0.0,  // lower left
				///* Second triangle */
				//-0.5, -0.5, 0.0,  // lower left
				// 0.5, -0.5, 0.0,  // lower right
				// 0.5,  0.5, 0.0,  // upper right
	pub fn new_quad() -> MeshBuilder {
		// TODO: use indices
		return MeshBuilder::new()
			.vertex_data(&[
				-0.5,  0.5, 0.0,  // upper left
				 0.5,  0.5, 0.0,  // upper right
				-0.5, -0.5, 0.0,  // lower left
				 0.5, -0.5, 0.0,  // lower right
			])
            .indices(&[
                0, 1, 2,
                1, 2, 3,
            ])
			.attribute(0, 3);
    }

	pub fn vertex_data(mut self, vertices: &[GLfloat]) -> MeshBuilder {
		for vertex in vertices {
			self.vertices.push(*vertex);
		}
		return self;
	}

	pub fn indices(mut self, indices: &[GLuint]) -> MeshBuilder {
		for index in indices {
			self.indices.push(*index);
		}
		return self;
	}

	// TODO: support named attributes, probably though a new function
	pub fn attribute(
		mut self,
		location: GLuint,
		component_amt: GLint
	) -> MeshBuilder {
		self.attributes.push(Attribute{ location, component_amt });
		return self;
	}

	pub fn build(self) -> Mesh {
		/* Create the vertex array object that will track this mesh */
		let vao: VertexArray = VertexArray::new();
		vao.bind();

		/* Calculate row size */
		let mut row_size: usize = 0;
		for attribute in &self.attributes {
			row_size += (attribute.component_amt as usize) * size_of::<GLfloat>();
        }

		/* Create vertex buffer and insert our data */
		let vbo_verts: Buffer = Buffer::new_array_buffer();
		vbo_verts.bind();
		vbo_verts.set_data_static_draw(&self.vertices);

		/* Specify the format of provided vertex data */
		let mut col_offset: usize = 0;
		for attribute in &self.attributes {
		    unsafe {
            gl::EnableVertexAttribArray(attribute.location);
            gl::VertexAttribPointer(
                attribute.location,
                attribute.component_amt,
                gl::FLOAT,  // attribute data type
                gl::FALSE,  // normalized
				row_size as GLint,
                col_offset as *const GLvoid);
		    }

			/* Increase column offset by the amount of space the previous column occupied */
			col_offset += (attribute.component_amt as usize) * size_of::<GLfloat>();
		}


        /* Create an index buffer if we were provided indices */
        let mut uses_indices: bool = false;
		let mut element_amt = 0;
        if self.indices.len() > 0 {
            /* Create index buffer */
		    let vbo_indices: Buffer = Buffer::new_element_array_buffer();
		    vbo_indices.bind();
		    vbo_indices.set_data_static_draw(&self.indices);
            uses_indices = true;

            /* Set element amount to be equal to the number of provided indices */
		    element_amt = self.indices.len();

            /* Unbind opengl objects */
		    vbo_verts.unbind();
		    vao.unbind();
		    vbo_indices.unbind();  // NOTE: this must be unbound AFTER the vao, otherwise we'll lose the vao reference
        } else {
            /* If there are no indices, calculate element amount from the number of vertices */
			element_amt = (self.vertices.len() * size_of::<GLfloat>()) / row_size;

            /* Unbind opengl objects */
		    vbo_verts.unbind();
		    vao.unbind();
        }

		/* Create and return our new mesh */
		// TODO: is it safe to let the vbo go out of scope and be deleted?
		// Opengl shouldn't let it actually be deleted since the vao still
		// references it
		return Mesh{
			vao,
            uses_indices,
			element_amt,
		};
	}
}
