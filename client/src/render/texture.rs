use crate::Error;

use std::path::Path;

pub struct TextureAtlas {
	id: gl::types::GLuint,
	pub size: u16,
}

impl TextureAtlas {
	pub fn new<P: AsRef<Path>>(file: P, depth: i32) -> Result<TextureAtlas, Error> {
		let img = image::open(file.as_ref())?.into_rgb8();

		let width = img.width() as i32;
		let height = img.height() as i32 / depth;

		let mut id = 0;
		unsafe {
			gl::CreateTextures(gl::TEXTURE_2D_ARRAY, 1, &mut id);
			gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

			gl::TextureStorage3D(id, 1, gl::RGB8, width, height, depth as i32);
			for i in 0..depth {
				let ptr = img.as_ptr().offset((width * height * 3 * i as i32) as isize);
				gl::TextureSubImage3D(
					id,
					0,
					0,
					0,
					i,
					width,
					height,
					1,
					gl::RGB,
					gl::UNSIGNED_BYTE,
					ptr as *const gl::types::GLvoid,
				);
			}
		}

		Ok(TextureAtlas {
			id,
			size: img.width() as u16,
		})
	}

	pub fn bind(&self, slot: u32) {
		unsafe {
			gl::BindTextureUnit(slot, self.id);
		}
	}
}

impl core::ops::Drop for TextureAtlas {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteTextures(1, &self.id);
		}
	}
}
