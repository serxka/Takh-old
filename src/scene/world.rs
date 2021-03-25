use crate::render::{
	mesh::{data, Mesh, Vertex},
	shader::Program,
	texture::TextureAtlas,
};

pub struct RenderChunks {
	pub shader: std::rc::Rc<Program>,
	pub atlas: TextureAtlas,
	meshes: Vec<Mesh<ChunkVertex>>,
}

impl RenderChunks {
	pub fn new(shader: std::rc::Rc<Program>, atlas: TextureAtlas) -> Self {
		Self {
			shader,
			meshes: vec![],
			atlas,
		}
	}

	pub fn add_mesh(&mut self, mesh: Mesh<ChunkVertex>) {
		self.meshes.push(mesh);
	}

	pub fn add_meshs(&mut self, mut meshes: Vec<Mesh<ChunkVertex>>) {
		self.meshes.append(&mut meshes);
	}

	pub fn render(&self) {
		self.shader.bind();
		for mesh in &self.meshes {
			mesh.render();
		}
	}
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ChunkVertex {
	pos: data::f32_3,
	tex: data::f32_2,
}

impl ChunkVertex {
	pub const fn new(d0: f32, d1: f32, d2: f32, d3: f32, d4: f32) -> Self {
		Self {
			pos: data::f32_3 { d0, d1, d2 },
			tex: data::f32_2 { d0: d3, d1: d4 },
		}
	}
}

impl Vertex for ChunkVertex {
	fn set_vertex_attrib() {
		let stride = std::mem::size_of::<Self>();
		let location = 0;
		let offset = 0;
		unsafe { data::f32_3::set_vertex_attrib(stride, location, offset) }
		let location = 1;
		let offset = offset + std::mem::size_of::<data::f32_3>();
		unsafe { data::f32_2::set_vertex_attrib(stride, location, offset) }
	}
}
