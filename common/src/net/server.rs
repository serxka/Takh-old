use uuid::Uuid;

pub enum ClientBound {
	Auth(Auth),
	WorldData,
	WorldUpdate(WorldUpdate),
}

pub enum Auth {
	LoginSuccess { entity: Uuid },
}

pub enum WorldUpdate {
	/// Sent when a block is changed, `pos` is the global position of the block that has been changed
	/// `block_id` is the new block state that has been defined in the world block palette list
	BlockChange { pos: (u64, u64, u64), block_id: u32 },
	/// Teleport an entity to a certain location, `pos` is an abosolute world coordinate
	EntityTeleport { entity: Uuid, pos: (u64, u64, u64) },
	/// Used when a entity moves less than 8 voxels, `pos` is a relative difference compared to the
	/// entitys' current position, `rot` is an abosolute rotation rather than a difference -- of pairs (yaw, pitch), if the
	/// distance is larger than 8 blocks use `WorldUpdate::EntityTeleport`
	EntityTransform {
		entity: Uuid,
		pos: (i16, i16, i16),
		rot: (u8, u8),
	},
	/// Change the rotation of an entity, `rot` is an abosolute rotation, of pairs (yaw, pitch)
	EntityRotation { entity: Uuid, rot: (u8, u8) },
}
