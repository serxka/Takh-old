use crate::render::{
	mesh::{data, Mesh, MeshBuilder, Vertex},
	shader::Program,
	texture::TextureAtlas,
};

use common::world::{chunk::Chunk, voxel::VoxelTexture};

pub fn mesh_builder(chunk: &Chunk) -> MeshBuilder<ChunkVertex> {
	let mut mesh = MeshBuilder::new();
	for y in 0..Chunk::HEIGHT {
		for z in 0..Chunk::DEPTH {
			for x in 0..Chunk::WIDTH {
				if !chunk.is_air(x, y, z) {
					let tex = match chunk.get_voxel(x, y, z).texture {
						VoxelTexture::Single { faces } => faces,
						_ => panic!("lazy"),
					};
					let ts = tex.size;
					let ti = tex.index;

					// Back face
					if z == 0 || chunk.is_air(x, y, z.saturating_sub(1)) {
						mesh.push_quad(
							&ChunkVertex::new(
								1 + x as u8,
								1 + y as u8,
								0 + z as u8,
								0,
								0,
								1,
								ts,
								ts,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								1 + y as u8,
								0 + z as u8,
								0,
								0,
								1,
								0,
								ts,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								0 + y as u8,
								0 + z as u8,
								0,
								0,
								1,
								0,
								0,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								0 + y as u8,
								0 + z as u8,
								0,
								0,
								1,
								ts,
								0,
								ti,
							),
						);
					}
					// Front face
					if z == Chunk::DEPTH - 1 || chunk.is_air(x, y, z.saturating_add(1)) {
						mesh.push_quad(
							&ChunkVertex::new(
								1 + x as u8,
								0 + y as u8,
								1 + z as u8,
								0,
								0,
								-1,
								ts,
								0,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								0 + y as u8,
								1 + z as u8,
								0,
								0,
								-1,
								0,
								0,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								1 + y as u8,
								1 + z as u8,
								0,
								0,
								-1,
								0,
								ts,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								1 + y as u8,
								1 + z as u8,
								0,
								0,
								-1,
								ts,
								ts,
								ti,
							),
						);
					}
					// Left face
					if x == 0 || chunk.is_air(x.saturating_sub(1), y, z) {
						mesh.push_quad(
							&ChunkVertex::new(
								0 + x as u8,
								1 + y as u8,
								0 + z as u8,
								-1,
								0,
								0,
								ts,
								ts,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								1 + y as u8,
								1 + z as u8,
								-1,
								0,
								0,
								0,
								ts,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								0 + y as u8,
								1 + z as u8,
								-1,
								0,
								0,
								0,
								0,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								0 + y as u8,
								0 + z as u8,
								-1,
								0,
								0,
								ts,
								0,
								ti,
							),
						);
					}
					// Right face
					if x == Chunk::WIDTH - 1 || chunk.is_air(x.saturating_add(1), y, z) {
						mesh.push_quad(
							&ChunkVertex::new(
								1 + x as u8,
								0 + y as u8,
								0 + z as u8,
								1,
								0,
								0,
								ts,
								0,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								0 + y as u8,
								1 + z as u8,
								1,
								0,
								0,
								0,
								0,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								1 + y as u8,
								1 + z as u8,
								1,
								0,
								0,
								0,
								ts,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								1 + y as u8,
								0 + z as u8,
								1,
								0,
								0,
								ts,
								ts,
								ti,
							),
						);
					}
					// Bottom face
					if y == 0 || chunk.is_air(x, y.saturating_sub(1), z) {
						mesh.push_quad(
							&ChunkVertex::new(
								0 + x as u8,
								0 + y as u8,
								1 + z as u8,
								0,
								1,
								0,
								ts,
								0,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								0 + y as u8,
								1 + z as u8,
								0,
								1,
								0,
								0,
								0,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								0 + y as u8,
								0 + z as u8,
								0,
								1,
								0,
								0,
								ts,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								0 + y as u8,
								0 + z as u8,
								0,
								1,
								0,
								ts,
								ts,
								ti,
							),
						);
					}
					// Top face
					if y == Chunk::HEIGHT - 1 || chunk.is_air(x, y.saturating_add(1), z) {
						mesh.push_quad(
							&ChunkVertex::new(
								0 + x as u8,
								1 + y as u8,
								0 + z as u8,
								0,
								-1,
								0,
								ts,
								ts,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								1 + y as u8,
								0 + z as u8,
								0,
								-1,
								0,
								0,
								ts,
								ti,
							),
							&ChunkVertex::new(
								1 + x as u8,
								1 + y as u8,
								1 + z as u8,
								0,
								-1,
								0,
								0,
								0,
								ti,
							),
							&ChunkVertex::new(
								0 + x as u8,
								1 + y as u8,
								1 + z as u8,
								0,
								-1,
								0,
								ts,
								0,
								ti,
							),
						);
					}
				}
			}
		}
	}
	mesh
}

pub struct RenderChunks {
	pub shader: std::rc::Rc<Program>,
	pub atlas: TextureAtlas,
	meshes: Vec<((f32, f32, f32), Mesh<ChunkVertex>)>,
}

impl RenderChunks {
	pub fn new(shader: std::rc::Rc<Program>, atlas: TextureAtlas) -> Self {
		Self {
			shader,
			meshes: vec![],
			atlas,
		}
	}

	pub fn add_mesh(&mut self, mesh: Mesh<ChunkVertex>, coord: (f32, f32, f32)) {
		self.meshes.push((coord, mesh));
	}

	pub fn render(&self) {
		self.shader.bind();
		for (coord, mesh) in &self.meshes {
			let mut transform = ultraviolet::mat::Mat4::identity();
			transform.translate(&ultraviolet::vec::Vec3::new(
				coord.0 as f32,
				coord.1 as f32,
				coord.2 as f32,
			));
			self.shader.set_uniform_mat4("u_model", transform);
			mesh.render();
		}
	}
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ChunkVertex {
	pos: data::u8_3,
	norm: data::i8_3,
	tex: data::u16_3,
}

impl ChunkVertex {
	pub const fn new(
		d0: u8,
		d1: u8,
		d2: u8,
		d3: i8,
		d4: i8,
		d5: i8,
		d6: u16,
		d7: u16,
		d8: u16,
	) -> Self {
		Self {
			pos: data::u8_3::new(d0, d1, d2),
			norm: data::i8_3::new(d3, d4, d5),
			tex: data::u16_3::new(d6, d7, d8),
		}
	}
}

impl Vertex for ChunkVertex {
	fn set_vertex_attrib() {
		let stride = std::mem::size_of::<Self>();
		let location = 0;
		let offset = 0;
		unsafe { data::u8_3::set_vertex_attrib(stride, location, offset) }
		let location = 1;
		let offset = offset + std::mem::size_of::<data::u8_3>();
		unsafe { data::i8_3::set_vertex_attrib(stride, location, offset) }
		let location = 2;
		let offset = offset + std::mem::size_of::<data::i8_3>();
		unsafe { data::u16_3::set_vertex_attrib(stride, location, offset) }
	}
}
