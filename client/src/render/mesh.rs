use std::marker::PhantomData;

pub trait Vertex {
	fn set_vertex_attrib();
}

pub struct Mesh<V: Vertex> {
	vbo: gl::types::GLuint, // vertex buffer object
	vao: gl::types::GLuint, // vertex array object
	vert_count: i32,        // number of vertices
	_vert: PhantomData<V>,  // type of vertex we have
}

impl<V: Vertex> Mesh<V> {
	pub fn new(data: &[V]) -> Self {
		let mut vbo = 0;
		unsafe {
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(data.len() * std::mem::size_of::<V>()) as gl::types::GLsizeiptr,
				data.as_ptr() as *const gl::types::GLvoid,
				gl::STATIC_DRAW,
			);
		}

		let mut vao = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			V::set_vertex_attrib();
		}

		Mesh {
			vbo,
			vao,
			vert_count: data.len() as i32,
			_vert: PhantomData,
		}
	}

	pub fn render(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, self.vert_count);
		}
	}
}

impl<V: Vertex> core::ops::Drop for Mesh<V> {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteVertexArrays(1, &self.vao);
			gl::DeleteBuffers(1, &self.vbo);
		}
	}
}

/// A helper to build a mesh vertex by vertex
pub struct MeshBuilder<V: Vertex> {
	vertices: Vec<V>,
}

impl<V: Vertex + Copy> MeshBuilder<V> {
	pub fn new() -> MeshBuilder<V> {
		MeshBuilder { vertices: vec![] }
	}

	pub fn push_verts(mut self, data: &[V]) -> Self {
		self.vertices.extend_from_slice(data);
		self
	}

	// push_quad( (0, 0) (1, 0) (1, 1) (0, 1) )
	pub fn push_quad(&mut self, v0: &V, v1: &V, v2: &V, v3: &V) {
		self.vertices.reserve(6);
		// tri 1
		self.vertices.push(*v0);
		self.vertices.push(*v2);
		self.vertices.push(*v1);
		// tri 2
		self.vertices.push(*v0);
		self.vertices.push(*v3);
		self.vertices.push(*v2);
	}

	pub fn build(self) -> Mesh<V> {
		Mesh::new(&self.vertices)
	}
}

// TODO: proc macro this
#[allow(dead_code)]
pub mod data {
	impl u8_2 {
		pub const fn new(d0: u8, d1: u8) -> Self {
			Self { d0, d1 }
		}

		pub unsafe fn set_vertex_attrib(stride: usize, location: usize, offset: usize) {
			gl::EnableVertexAttribArray(location as gl::types::GLuint);
			gl::VertexAttribPointer(
				location as gl::types::GLuint,
				2,                 // number of componenets
				gl::UNSIGNED_BYTE, // data type
				gl::FALSE,         // normalised
				stride as gl::types::GLint,
				offset as *const gl::types::GLvoid,
			);
		}
	}

	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone, Debug)]
	#[repr(C, packed)]
	pub struct i8_2 {
		pub d0: i8,
		pub d1: i8,
	}

	impl i8_2 {
		pub const fn new(d0: i8, d1: i8) -> Self {
			Self { d0, d1 }
		}

		pub unsafe fn set_vertex_attrib(stride: usize, location: usize, offset: usize) {
			gl::EnableVertexAttribArray(location as gl::types::GLuint);
			gl::VertexAttribPointer(
				location as gl::types::GLuint,
				2,         // number of componenets
				gl::BYTE,  // data type
				gl::FALSE, // normalised
				stride as gl::types::GLint,
				offset as *const gl::types::GLvoid,
			);
		}
	}

	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone, Debug)]
	#[repr(C, packed)]
	pub struct u8_3 {
		pub d0: u8,
		pub d1: u8,
		pub d2: u8,
	}

	impl u8_3 {
		pub const fn new(d0: u8, d1: u8, d2: u8) -> Self {
			Self { d0, d1, d2 }
		}

		pub unsafe fn set_vertex_attrib(stride: usize, location: usize, offset: usize) {
			gl::EnableVertexAttribArray(location as gl::types::GLuint);
			gl::VertexAttribPointer(
				location as gl::types::GLuint,
				3,                 // number of componenets
				gl::UNSIGNED_BYTE, // data type
				gl::FALSE,         // normalised
				stride as gl::types::GLint,
				offset as *const gl::types::GLvoid,
			);
		}
	}

	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone, Debug)]
	#[repr(C, packed)]
	pub struct i8_3 {
		pub d0: i8,
		pub d1: i8,
		pub d2: i8,
	}

	impl i8_3 {
		pub const fn new(d0: i8, d1: i8, d2: i8) -> Self {
			Self { d0, d1, d2 }
		}

		pub unsafe fn set_vertex_attrib(stride: usize, location: usize, offset: usize) {
			gl::EnableVertexAttribArray(location as gl::types::GLuint);
			gl::VertexAttribPointer(
				location as gl::types::GLuint,
				3,         // number of componenets
				gl::BYTE,  // data type
				gl::FALSE, // normalised
				stride as gl::types::GLint,
				offset as *const gl::types::GLvoid,
			);
		}
	}

	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone, Debug)]
	#[repr(C, packed)]
	pub struct u16_3 {
		pub d0: u16,
		pub d1: u16,
		pub d2: u16,
	}

	impl u16_3 {
		pub const fn new(d0: u16, d1: u16, d2: u16) -> Self {
			Self { d0, d1, d2 }
		}

		pub unsafe fn set_vertex_attrib(stride: usize, location: usize, offset: usize) {
			gl::EnableVertexAttribArray(location as gl::types::GLuint);
			gl::VertexAttribPointer(
				location as gl::types::GLuint,
				3,                  // number of componenets
				gl::UNSIGNED_SHORT, // data type
				gl::FALSE,          // normalised
				stride as gl::types::GLint,
				offset as *const gl::types::GLvoid,
			);
		}
	}

	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone, Debug)]
	#[repr(C, packed)]
	pub struct f32_2 {
		pub d0: f32,
		pub d1: f32,
	}

	impl f32_2 {
		pub const fn new(d0: f32, d1: f32) -> Self {
			Self { d0, d1 }
		}

		pub unsafe fn set_vertex_attrib(stride: usize, location: usize, offset: usize) {
			gl::EnableVertexAttribArray(location as gl::types::GLuint);
			gl::VertexAttribPointer(
				location as gl::types::GLuint,
				2,         // number of componenets
				gl::FLOAT, // data type
				gl::FALSE, // normalised
				stride as gl::types::GLint,
				offset as *const gl::types::GLvoid,
			);
		}
	}

	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone, Debug)]
	#[repr(C, packed)]
	pub struct u8_2 {
		pub d0: u8,
		pub d1: u8,
	}

	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone, Debug)]
	#[repr(C, packed)]
	pub struct f32_3 {
		pub d0: f32,
		pub d1: f32,
		pub d2: f32,
	}

	impl f32_3 {
		pub const fn new(d0: f32, d1: f32, d2: f32) -> Self {
			Self { d0, d1, d2 }
		}

		pub unsafe fn set_vertex_attrib(stride: usize, location: usize, offset: usize) {
			gl::EnableVertexAttribArray(location as gl::types::GLuint);
			gl::VertexAttribPointer(
				location as gl::types::GLuint,
				3,         // number of componenets
				gl::FLOAT, // data type
				gl::FALSE, // normalised
				stride as gl::types::GLint,
				offset as *const gl::types::GLvoid,
			);
		}
	}
}
