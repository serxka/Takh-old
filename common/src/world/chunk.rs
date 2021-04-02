use super::voxel::{Voxel, AIR_VOXEL};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaletteId(u8);

impl PaletteId {
	#[inline(always)]
	pub fn new(id: u8) -> PaletteId {
		PaletteId(id)
	}

	#[inline(always)]
	pub fn value(&self) -> u8 {
		self.0
	}
}

#[derive(Clone)]
pub struct Palette {
	last_free: usize,
	types: Vec<Voxel>,
}

impl Palette {
	pub const MAX_SIZE: usize = std::mem::size_of::<u8>() * 8;

	pub fn new() -> Palette {
		let last_free = 1;
		let types = vec![AIR_VOXEL; Self::MAX_SIZE];

		Palette { last_free, types }
	}

	pub fn palette_id(&self, voxel: &Voxel) -> Option<PaletteId> {
		self.types
			.iter()
			.enumerate()
			.find(|(_, v)| if *v == voxel { true } else { false })
			.map(|(i, _)| PaletteId::new(i as u8))
	}

	pub fn add_voxel(&mut self, voxel: Voxel) {
		for i in self.last_free..Self::MAX_SIZE {
			if self.types[i] == AIR_VOXEL {
				self.types[i] = voxel;
				self.last_free += 1;
				break;
			}
		}
	}
}

impl std::ops::Index<PaletteId> for Palette {
	type Output = Voxel;

	fn index(&self, i: PaletteId) -> &Self::Output {
		&self.types[i.value() as usize]
	}
}

pub struct Chunk {
	pub coord: (i32, i32, i32),
	pub palette: Palette,
	voxels: Vec<PaletteId>,
}

impl Chunk {
	pub const DEPTH: usize = 32;
	pub const HEIGHT: usize = 64;
	pub const WIDTH: usize = 32;

	pub fn new(coord: (i32, i32, i32), palette: Palette) -> Chunk {
		let voxels = vec![PaletteId::new(0); Self::WIDTH * Self::HEIGHT * Self::DEPTH];
		Chunk {
			coord,
			palette,
			voxels,
		}
	}

	pub fn palette_id(&self, voxel: &Voxel) -> Option<PaletteId> {
		self.palette.palette_id(voxel)
	}

	#[inline(always)]
	pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, id: PaletteId) {
		if x >= Self::WIDTH || y >= Self::HEIGHT || x >= Self::DEPTH {
			return;
		}
		let idx = (y * Self::WIDTH * Self::DEPTH) + (z * Self::WIDTH) + x;
		self.voxels[idx] = id;
	}

	#[inline(always)]
	pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> &Voxel {
		if x >= Self::WIDTH || y >= Self::HEIGHT || x >= Self::DEPTH {
			return &self.palette[PaletteId::new(0)];
		}
		let idx = (y * Self::WIDTH * Self::DEPTH) + (z * Self::WIDTH) + x;
		&self.palette[self.voxels[idx]]
	}

	#[inline(always)]
	pub fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
		if x >= Self::WIDTH || y >= Self::HEIGHT || z >= Self::DEPTH {
			return false;
		}
		let idx = (y * Self::WIDTH * Self::DEPTH) + (z * Self::WIDTH) + x;
		self.voxels[idx] == PaletteId::new(0)
	}
}
