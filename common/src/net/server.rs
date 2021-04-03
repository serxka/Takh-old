use crate::net::{
	types::{EntityID, Position, Rotation, VPosition},
	world::WorldData,
};

use uuid::Uuid;

pub enum ClientBound {
	Auth(Auth),
	Data(WorldData),
	Update(WorldUpdate),
}

pub enum Auth {
	/// Sent to client when successful login
	LoginSuccess { entity: Uuid },
	/// Client should assume connect has been closed when this is sent.
	Disconnect { message: String },
}

pub enum WorldUpdate {
	/// Sent when a block is changed, `pos` is the global position of the voxel that has been changed
	/// `block_id` is the new voxel state that has been defined in the world voxel palette list
	BlockChange { pos: VPosition, voxel_id: u32 },
	/// Teleport an entity to a certain location, `pos` is an abosolute world coordinate
	EntityTeleport { entity: EntityID, pos: Position },
	/// Used when a entity moves less than 8 voxels, `pos` is a relative difference compared to the
	/// entitys' current position, `rot` is an abosolute rotation rather than a difference -- of pairs (yaw, pitch), if the
	/// distance is larger than 8 blocks use `WorldUpdate::EntityTeleport`
	EntityTransform {
		entity: EntityID,
		pos: (i16, i16, i16),
		rot: Rotation,
	},
	/// Spawn Player
	SpawnPlayer {
		entity: EntityID,
		pos: Position,
		rot: Rotation,
	},
	/// Tell's the client to update their copy of a chunk palette with a voxel from the world palette
	UpdateChunkPalette {
		chunk_pos: VPosition,
		chunk_voxel: u16,
		world_voxel: u32,
	},
}
