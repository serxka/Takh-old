use crate::render::{
	mesh::{data, Mesh, Vertex},
	shader::Program,
};

pub struct Framebuffer {
	fbo: gl::types::GLuint,
	rbo: gl::types::GLuint,
	texture_buf: gl::types::GLuint,
	quad: Mesh<PostprocessVertex>,
	shader: Program,
}

static BASIC_VS: &'static str = include_str!("../../../res/client/shaders/postprocess/basic.vs");
static BASIC_FS: &'static str = include_str!("../../../res/client/shaders/postprocess/basic.fs");

impl Framebuffer {
	pub fn new(w: u32, h: u32) -> Framebuffer {
		let quad = Mesh::new(&SCREEN_QUAD);
		let shader = Program::from_vert_and_frag(BASIC_VS, BASIC_FS).unwrap();

		let mut fbo = 0;
		unsafe {
			gl::GenFramebuffers(1, &mut fbo);
			gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
		}
		let mut texture_buf = 0;
		unsafe {
			gl::GenTextures(1, &mut texture_buf);
			gl::BindTexture(gl::TEXTURE_2D, texture_buf);
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGB8 as i32,
				w as i32,
				h as i32,
				0,
				gl::RGB,
				gl::UNSIGNED_BYTE,
				std::ptr::null(),
			);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::FramebufferTexture2D(
				gl::FRAMEBUFFER,
				gl::COLOR_ATTACHMENT0,
				gl::TEXTURE_2D,
				texture_buf,
				0,
			)
		}
		let mut rbo = 0;
		unsafe {
			gl::GenRenderbuffers(1, &mut rbo);
			gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
			gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, w as i32, h as i32);
			gl::FramebufferRenderbuffer(
				gl::FRAMEBUFFER,
				gl::DEPTH_STENCIL_ATTACHMENT,
				gl::RENDERBUFFER,
				rbo,
			);
			if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
				log::error!("Framebuffer is not complete");
			}
		}

		Framebuffer {
			fbo,
			rbo,
			texture_buf,
			quad,
			shader,
		}
	}

	/// Set shader program to use as our postprocessing effect, a value of `None` will
	/// reset to default shader program
	pub fn shader_program(&mut self, program: Option<Program>) {
		if program.is_some() {
			self.shader = program.unwrap();
			self.shader.bind();
			self.shader.set_uniform_int("screen_tex", 0);
		}
		self.shader = Program::from_vert_and_frag(BASIC_VS, BASIC_FS).unwrap();
	}

	pub fn resize(&mut self, w: u32, h: u32) {
		unsafe {
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGB8 as i32,
				w as i32,
				h as i32,
				0,
				gl::RGB,
				gl::UNSIGNED_BYTE,
				std::ptr::null(),
			);
			gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, w as i32, h as i32);
		}
		// unimplemented!()
	}

	/// Bind our framebuffer for off screen drawing of screen
	pub fn bind(&self) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
			gl::Enable(gl::DEPTH_TEST);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
	}

	/// Draw our texture and render that to our main frame buffer
	pub fn draw(&self) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
			gl::Disable(gl::DEPTH_TEST);
		}
		self.shader.bind();
		self.quad.render();
	}
}

impl std::ops::Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteRenderbuffers(1, &self.rbo);
			gl::DeleteTextures(1, &self.texture_buf);
			gl::DeleteFramebuffers(1, &self.fbo);
		}
	}
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct PostprocessVertex {
	pos: data::i8_2,
	tex: data::u8_2,
}

const SCREEN_QUAD: [PostprocessVertex; 6] = [
	PostprocessVertex::new(-1, 1, 0, 1),
	PostprocessVertex::new(-1, -1, 0, 0),
	PostprocessVertex::new(1, -1, 1, 0),
	PostprocessVertex::new(-1, 1, 0, 1),
	PostprocessVertex::new(1, -1, 1, 0),
	PostprocessVertex::new(1, 1, 1, 1),
];

impl PostprocessVertex {
	pub const fn new(d0: i8, d1: i8, d2: u8, d3: u8) -> Self {
		Self {
			pos: data::i8_2::new(d0, d1),
			tex: data::u8_2::new(d2, d3),
		}
	}
}

impl Vertex for PostprocessVertex {
	fn set_vertex_attrib() {
		let stride = std::mem::size_of::<Self>();
		let location = 0;
		let offset = 0;
		unsafe { data::i8_2::set_vertex_attrib(stride, location, offset) }
		let location = 1;
		let offset = offset + std::mem::size_of::<data::i8_2>();
		unsafe { data::u8_2::set_vertex_attrib(stride, location, offset) }
	}
}
