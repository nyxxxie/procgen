// TODO:  create a Vertex object where we can set position, color,  texture
// coords, etc.  These will automaticaly be linked to attributes.
// Maybe expose the first raw interface and then put this on top?
//
// Also, create a premade mesh generator (colored cube, textured cube, cylinder, quad, etc)
extern crate gl;

use std::mem::size_of;
use engine::buffer::Buffer;
use engine::shader::Program;
use engine::vao::VertexArray;
use gl::types::{ GLuint, GLint, GLvoid };

/// Represents a mesh of vertices.
pub struct Mesh {
    vao: VertexArray,
	shader: Program,
}

impl Mesh {
	pub fn draw(&self, mode: GLuint) {
		self.vao.bind();
		self.shader.activate();
		unsafe {
            gl::DrawArrays(
                mode,  // mode
                0,  // starting index
                3);  // number of indices to be rendered
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
	vertices: Vec<f32>,
	indices: Vec<u32>,
	attributes: Vec<Attribute>,
	shader: Option<Program>,
}

impl MeshBuilder {
	pub fn new() -> MeshBuilder {
		return MeshBuilder{
			vertices: Vec::new(),
			indices: Vec::new(),
			attributes: Vec::new(),
			shader: None,
		};
	}

	pub fn vertex_data(mut self, vertices: &[f32]) -> MeshBuilder {
		for vertex in vertices {
			self.vertices.push(*vertex);
		}
		return self;
	}

	pub fn indices(mut self, indices: &[u32]) -> MeshBuilder {
		for index in indices {
			self.indices.push(*index);
		}
		return self;
	}

	pub fn attribute(
		mut self,
		location: GLuint,
		component_amt: GLint
	) -> MeshBuilder {
		self.attributes.push(Attribute{ location, component_amt });
		return self;
	}

	pub fn shader(mut self, shader: Program) -> MeshBuilder {
		self.shader = Some(shader);
		return self;
	}

	pub fn build(self) -> Mesh {
		/* Create the vertex array object that will track this mesh */
		let vao: VertexArray = VertexArray::new();
		vao.bind();

		/* Create vertex buffer and insert our data */
		let vbo: Buffer = Buffer::new_array_buffer();
		vbo.bind();
		vbo.set_data_static_draw(&self.vertices);

		/* Calculate row size */
		let mut row_size: usize = 0;
		for attribute in &self.attributes {
			row_size += (attribute.component_amt as usize) * size_of::<f32>();
        }

		/* Set attributes */
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
			col_offset += (attribute.component_amt as usize) * size_of::<f32>();
		}

		/* Unbind our opengl objects for safety */
		vbo.unbind();
		vao.unbind();

		/* Create and return our new mesh */
		// TODO: is it safe to let the vbo go out of scope and be deleted?
		// Opengl shouldn't let it actually be deleted since the vao still
		// references it
		return Mesh{ vao, shader: self.shader.unwrap() };
	}
}
