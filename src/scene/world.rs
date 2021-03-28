use crate::render::{
	mesh::{data, Mesh, MeshBuilder, Vertex},
	shader::Program,
	texture::TextureAtlas,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Voxel {
	pub atlas_idx: u16,
	pub is_solid: bool,
}

unsafe fn _size_assert() {
	std::mem::transmute::<Voxel, u32>(Voxel {
		atlas_idx: 0,
		is_solid: false,
	});
}

impl Voxel {
	pub fn new(atlas_idx: u16) -> Voxel {
		Voxel {
			atlas_idx,
			is_solid: true,
		}
	}
}

impl std::default::Default for Voxel {
	fn default() -> Voxel {
		Voxel {
			atlas_idx: 0,
			is_solid: false,
		}
	}
}

pub struct Chunk {
	index: (u32, u32, u32),
	voxels: Vec<Voxel>,
}

impl Chunk {
	const WIDTH: usize = 32;
	const HEIGHT: usize = 32;
	const DEPTH: usize = 64;

	pub fn new(index: (u32, u32, u32)) -> Chunk {
		let voxels = vec![0u32; Self::WIDTH * Self::HEIGHT * Self::DEPTH];
		let voxels = unsafe { std::mem::transmute(voxels) };

		Chunk { index, voxels }
	}

	/// x, y, z
	#[inline(always)]
	pub fn set_voxel(&mut self, pos: (usize, usize, usize), voxel: Voxel) {
		if pos.0 >= Self::WIDTH || pos.1 >= Self::HEIGHT || pos.2 >= Self::DEPTH {
			return;
		}
		let idx = (pos.1 * Self::WIDTH * Self::DEPTH) + (pos.2 * Self::WIDTH) + pos.0;
		self.voxels[idx] = voxel;
	}

	#[inline(always)]
	pub fn is_solid(&self, pos: (usize, usize, usize)) -> bool {
		if pos.0 >= Self::WIDTH || pos.1 >= Self::HEIGHT || pos.2 >= Self::DEPTH {
			return false;
		}
		let idx = (pos.1 * Self::WIDTH * Self::DEPTH) + (pos.2 * Self::WIDTH) + pos.0;
		self.voxels[idx].is_solid
	}

	pub fn mesh_builder(&self) -> MeshBuilder<ChunkVertex> {
		let mut mesh = MeshBuilder::new();
		for y in 0..Self::HEIGHT {
			for z in 0..Self::DEPTH {
				for x in 0..Self::WIDTH {
					if self.is_solid((x, y, z)) {
						// Back face
						if !self.is_solid((x, y, z.saturating_sub(1))) {
							mesh.push_quad(
								&ChunkVertex::new(1 + x as u8, 1 + y as u8, 0 + z as u8, 32, 32, 0),
								&ChunkVertex::new(0 + x as u8, 1 + y as u8, 0 + z as u8, 0, 32, 0),
								&ChunkVertex::new(0 + x as u8, 0 + y as u8, 0 + z as u8, 0, 0, 0),
								&ChunkVertex::new(1 + x as u8, 0 + y as u8, 0 + z as u8, 32, 0, 0),
							);
						}
						// Front face
						if !self.is_solid((x, y, z.saturating_add(1))) {
							mesh.push_quad(
								&ChunkVertex::new(1 + x as u8, 0 + y as u8, 1 + z as u8, 32, 0, 0),
								&ChunkVertex::new(0 + x as u8, 0 + y as u8, 1 + z as u8, 0, 0, 0),
								&ChunkVertex::new(0 + x as u8, 1 + y as u8, 1 + z as u8, 0, 32, 0),
								&ChunkVertex::new(1 + x as u8, 1 + y as u8, 1 + z as u8, 32, 32, 0),
							);
						}
						// Left face
						if !self.is_solid((x.saturating_sub(1), y, z)) {
							mesh.push_quad(
								&ChunkVertex::new(0 + x as u8, 1 + y as u8, 0 + z as u8, 32, 32, 0),
								&ChunkVertex::new(0 + x as u8, 1 + y as u8, 1 + z as u8, 0, 32, 0),
								&ChunkVertex::new(0 + x as u8, 0 + y as u8, 1 + z as u8, 0, 0, 0),
								&ChunkVertex::new(0 + x as u8, 0 + y as u8, 0 + z as u8, 32, 0, 0),
							);
						}
						// Right face
						if !self.is_solid((x.saturating_add(1), y, z)) {
							mesh.push_quad(
								&ChunkVertex::new(1 + x as u8, 0 + y as u8, 0 + z as u8, 32, 0, 0),
								&ChunkVertex::new(1 + x as u8, 0 + y as u8, 1 + z as u8, 0, 0, 0),
								&ChunkVertex::new(1 + x as u8, 1 + y as u8, 1 + z as u8, 0, 32, 0),
								&ChunkVertex::new(1 + x as u8, 1 + y as u8, 0 + z as u8, 32, 32, 0),
							);
						}
						// Top face
						if !self.is_solid((x, y.saturating_add(1), z)) {
							mesh.push_quad(
								&ChunkVertex::new(0 + x as u8, 1 + y as u8, 0 + z as u8, 32, 32, 0),
								&ChunkVertex::new(1 + x as u8, 1 + y as u8, 0 + z as u8, 0, 32, 0),
								&ChunkVertex::new(1 + x as u8, 1 + y as u8, 1 + z as u8, 0, 0, 0),
								&ChunkVertex::new(0 + x as u8, 1 + y as u8, 1 + z as u8, 32, 0, 0),
							);
						}
						// Bottom face
						if !self.is_solid((x, y.saturating_sub(1), z)) {
							mesh.push_quad(
								&ChunkVertex::new(0 + x as u8, 0 + y as u8, 1 + z as u8, 32, 0, 0),
								&ChunkVertex::new(1 + x as u8, 0 + y as u8, 1 + z as u8, 0, 0, 0),
								&ChunkVertex::new(1 + x as u8, 0 + y as u8, 0 + z as u8, 0, 32, 0),
								&ChunkVertex::new(0 + x as u8, 0 + y as u8, 0 + z as u8, 32, 32, 0),
							);
						}	
					}

				}
			}
		}
		mesh
	}
}

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

	pub fn add_meshes(&mut self, mut meshes: Vec<Mesh<ChunkVertex>>) {
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
	pos: data::u8_3,
	tex: data::u16_3,
}

impl ChunkVertex {
	pub const fn new(d0: u8, d1: u8, d2: u8, d3: u16, d4: u16, d5: u16) -> Self {
		Self {
			pos: data::u8_3::new(d0, d1, d2),
			tex: data::u16_3::new(d3, d4, d5),
		}
	}
	
	// Back face
	pub const BACK_FACE: [ChunkVertex; 6] = [
		ChunkVertex::new(1, 1, 0, 32, 0, 0),
		ChunkVertex::new(1, 0, 0, 32, 32, 0),
		ChunkVertex::new(0, 0, 0, 0, 32, 0),
		ChunkVertex::new(0, 0, 0, 0, 32, 0),
		ChunkVertex::new(0, 1, 0, 0, 0, 0),
		ChunkVertex::new(1, 1, 0, 32, 0, 0),
	];
	// Front face
	pub const FRONT_FACE: [ChunkVertex; 6] = [
		ChunkVertex::new(0, 0, 1, 0, 32, 0),
		ChunkVertex::new(1, 0, 1, 32, 32, 0),
		ChunkVertex::new(1, 1, 1, 32, 0, 0),
		ChunkVertex::new(1, 1, 1, 32, 0, 0),
		ChunkVertex::new(0, 1, 1, 0, 0, 0),
		ChunkVertex::new(0, 0, 1, 0, 32, 0),
	];
	// Left face
	pub const LEFT_FACE: [ChunkVertex; 6] = [
		ChunkVertex::new(0, 0, 0, 0, 32, 0),
		ChunkVertex::new(0, 0, 1, 32, 32, 0),
		ChunkVertex::new(0, 1, 1, 32, 0, 0),
		ChunkVertex::new(0, 1, 1, 32, 0, 0),
		ChunkVertex::new(0, 1, 0, 0, 0, 0),
		ChunkVertex::new(0, 0, 0, 0, 32, 0),
	];
	// Right face
	pub const RIGHT_FACE: [ChunkVertex; 6] = [
		ChunkVertex::new(1, 1, 1, 32, 0, 0),
		ChunkVertex::new(1, 0, 1, 32, 32, 0),
		ChunkVertex::new(1, 0, 0, 0, 32, 0),
		ChunkVertex::new(1, 0, 0, 0, 32, 0),
		ChunkVertex::new(1, 1, 0, 0, 0, 0),
		ChunkVertex::new(1, 1, 1, 32, 0, 0),
	];
	// Bottom face
	pub const BOTTOM_FACE: [ChunkVertex; 6] = [
		ChunkVertex::new(1, 0, 1, 32, 0, 0),
		ChunkVertex::new(0, 0, 1, 0, 0, 0),
		ChunkVertex::new(0, 0, 0, 0, 32, 0),
		ChunkVertex::new(0, 0, 0, 0, 32, 0),
		ChunkVertex::new(1, 0, 0, 32, 32, 0),
		ChunkVertex::new(1, 0, 1, 32, 0, 0),
	];
	// Top face
	pub const TOP_FACE: [ChunkVertex; 6] = [
		ChunkVertex::new(0, 1, 0, 0, 0, 0),
		ChunkVertex::new(0, 1, 1, 0, 32, 0),
		ChunkVertex::new(1, 1, 1, 32, 32, 0),
		ChunkVertex::new(1, 1, 1, 32, 32, 0),
		ChunkVertex::new(1, 1, 0, 32, 0, 0),
		ChunkVertex::new(0, 1, 0, 0, 0, 0),
	];
}

impl Vertex for ChunkVertex {
	fn set_vertex_attrib() {
		let stride = std::mem::size_of::<Self>();
		let location = 0;
		let offset = 0;
		unsafe { data::u8_3::set_vertex_attrib(stride, location, offset) }
		let location = 1;
		let offset = offset + std::mem::size_of::<data::u8_3>();
		unsafe { data::u16_3::set_vertex_attrib(stride, location, offset) }
	}
}
