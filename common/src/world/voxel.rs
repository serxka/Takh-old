#[derive(Clone, PartialEq, Eq)]
pub struct Voxel {
	pub is_air: bool,
	pub collide: bool,
	pub mesh: VoxelMesh,
	#[cfg(feature = "client")]
	pub texture: VoxelTexture,
}

pub const AIR_VOXEL: Voxel = Voxel {
	is_air: true,
	collide: false,
	mesh: VoxelMesh::Nil,
	#[cfg(feature = "client")]
	texture: VoxelTexture::None,
};

#[cfg(feature = "client")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TextureId {
	pub size: u16,
	pub index: u16,
}

#[cfg(feature = "client")]
impl TextureId {
	pub fn new(size: u16, index: u16) -> TextureId {
		TextureId { size, index }
	}
}

#[cfg(feature = "client")]
#[derive(Clone, PartialEq, Eq)]
pub enum VoxelTexture {
	/// No texture (Nil mesh type or just a pure white block)
	None,
	/// The same face wrapped entire way around voxel
	Single { faces: TextureId },
	/// Same face for all sides except top
	Top { sides: TextureId, top: TextureId },
	/// Same face for all sides except top and bottom
	Sides {
		sides: TextureId,
		top: TextureId,
		bottom: TextureId,
	},
	/// Different texture for every face
	All {
		top: TextureId,
		bottom: TextureId,
		right: TextureId,
		left: TextureId,
		front: TextureId,
		back: TextureId,
	},
}

impl Voxel {
	#[cfg(not(feature = "client"))]
	pub fn new_full() -> Voxel {
		Voxel {
			is_air: false,
			collide: true,
			mesh: VoxelMesh::Full,
		}
	}

	#[cfg(feature = "client")]
	pub fn new_full(texture: VoxelTexture) -> Voxel {
		Voxel {
			is_air: false,
			collide: true,
			mesh: VoxelMesh::Full,
			texture,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VoxelMesh {
	/// A full filled standard voxel
	Full,
	/// A half voxel
	Half,
	/// Slice of a fullly filled voxel, determined by a ratio of `u8 / (1.0*256.0)`
	Fraction(u8),
	/// A voxel with zero volume (trigger or something)
	Nil,
	/// Completely different stucture from normal voxel
	Model(usize),
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VoxelDirection {
	/// y+
	Up,
	/// y-
	Down,
	/// x+
	Right,
	/// x-
	Left,
	/// z+
	Away,
	/// z-
	Towards,
}
