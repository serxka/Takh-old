use crate::net::types::VPosition;

pub enum WorldData {
	WorldPalette {},
	ChunkPalette {},
	ChunkData { pos: VPosition },
}
