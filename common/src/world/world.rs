use crate::world::chunk::{Chunk, Palette, PaletteId};

use noise::{NoiseFn, OpenSimplex};

pub fn generate_chunk(cx: i32, cy: i32, cz: i32, palette: Palette) -> Chunk {
	let mut chunk = Chunk::new((cx, cy, cz), palette);
	let simplex = OpenSimplex::new();
	for z in 0..Chunk::DEPTH {
		for x in 0..Chunk::WIDTH {
			let ax = x as i32 + (cx * Chunk::WIDTH as i32);
			let az = z as i32 + (cz * Chunk::DEPTH as i32);
			let mut noise = simplex.get([ax as f64 / 512.0, az as f64 / 512.0]) * 24.0;
			noise += simplex.get([ax as f64 / 64.0, az as f64 / 64.0]) * 30.0;
			noise += 24.0;
			for y in 0..noise as usize {
				chunk.set_voxel(x, y, z, PaletteId::new(1));
			}
		}
	}
	chunk
}
