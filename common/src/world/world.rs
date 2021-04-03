use crate::world::chunk::{Chunk, Palette, PaletteId};

use noise::{NoiseFn, OpenSimplex};

pub fn generate_chunk(cx: i32, cy: i32, cz: i32, palette: Palette) -> Chunk {
	let mut chunk = Chunk::new((cx, cy, cz), palette);
	let simplex = OpenSimplex::new();
	for z in 0..Chunk::DEPTH {
		// let ax = x as i32 + (cx * Chunk::WIDTH as i32);
		// let az = z as i32 + (cz * Chunk::DEPTH as i32);
		// let mut noise = simplex.get([ax as f64 / 512.0, az as f64 / 512.0]) * 24.0;
		// noise += simplex.get([ax as f64 / 64.0, az as f64 / 64.0]) * 30.0;
		// noise += 24.0;
		// for y in 0..noise as usize {
		// 	chunk.set_voxel(x, y, z, PaletteId::new(1));
		// }
		for x in 0..Chunk::WIDTH {
			for y in 0..Chunk::HEIGHT {
				let value = simplex.get([
					((cx * Chunk::WIDTH as i32) as f64 + x as f64) / 16.0,
					((cy * Chunk::HEIGHT as i32) as f64 + y as f64) / 16.0,
					((cz * Chunk::DEPTH as i32) as f64 + z as f64) / 16.0,
				]);
				let voxel = if value > 0.2 {
					PaletteId::new(1)
				} else {
					PaletteId::new(0)
				};
				chunk.set_voxel(x, y, z, voxel);
			}
		}
	}
	chunk
}
