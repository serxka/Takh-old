use crate::net::VPosition;

pub enum WorldData {
	WorldPalette {},
	ChunkPalette {},
	ChunkData { pos: VPosition },
}
