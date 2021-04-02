mod error;
pub mod mesh;
pub mod shader;
pub mod texture;

pub use error::RenderError;

pub struct Renderer {
	pub context: glutin::RawContext<glutin::PossiblyCurrent>,
}

impl Renderer {
	pub fn new(
		context: glutin::RawContext<glutin::PossiblyCurrent>,
		win_size: [u32; 2],
	) -> Renderer {
		let _ = gl::load_with(|s| context.get_proc_address(s));
		unsafe {
			gl::Viewport(0, 0, win_size[0] as i32, win_size[1] as i32);
			gl::ClearColor(0.257, 0.527, 0.5, 1.0);
			gl::Enable(gl::CULL_FACE);
			gl::Enable(gl::DEPTH_TEST);
			gl::CullFace(gl::BACK);
		}

		Renderer { context }
	}

	pub fn viewport(&mut self, x: i32, y: i32, w: u32, h: u32) {
		unsafe {
			gl::Viewport(x, y, w as i32, h as i32);
		}
	}

	pub fn clear(&self) {
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
	}
}
