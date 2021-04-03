mod error;
pub mod framebuffer;
pub mod mesh;
pub mod shader;
pub mod texture;

pub use error::RenderError;

use framebuffer::Framebuffer;

pub struct Renderer {
	pub context: glutin::RawContext<glutin::PossiblyCurrent>,
	pub framebuffer: Framebuffer,
}

impl Renderer {
	pub fn new(context: glutin::RawContext<glutin::PossiblyCurrent>, win_size: [u32; 2]) -> Renderer {
		let _ = gl::load_with(|s| context.get_proc_address(s));
		if cfg!(debug_assertions) {
			unsafe {
				gl::Enable(gl::DEBUG_OUTPUT);
				gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
				gl::DebugMessageCallback(Some(gl_debug_callback), std::ptr::null());
				gl::DebugMessageControl(
					gl::DEBUG_SOURCE_API,
					gl::DEBUG_TYPE_ERROR,
					gl::DEBUG_SEVERITY_MEDIUM,
					0,
					std::ptr::null(),
					gl::TRUE,
				);
			}
		}

		unsafe {
			gl::Viewport(0, 0, win_size[0] as i32, win_size[1] as i32);
			gl::ClearColor(0.257, 0.527, 0.5, 1.0);
			gl::Enable(gl::CULL_FACE);
			gl::CullFace(gl::BACK);
		}
		let framebuffer = Framebuffer::new(win_size[0], win_size[1]);

		Renderer { context, framebuffer }
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

use gl::types::{GLchar, GLenum, GLsizei, GLuint, GLvoid};
use std::ffi::CStr;

extern "system" fn gl_debug_callback(
	source: GLenum,
	gltype: GLenum,
	id: GLuint,
	severity: GLenum,
	_length: GLsizei,
	message: *const GLchar,
	_user_param: *mut GLvoid,
) {
	fn source_str(source: GLenum) -> &'static str {
		match source {
			gl::DEBUG_SOURCE_API => "API",
			gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
			gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
			gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
			gl::DEBUG_SOURCE_APPLICATION => "Application",
			gl::DEBUG_SOURCE_OTHER => "Other",
			_ => "Unknown",
		}
	}

	fn type_str(gltype: GLenum) -> &'static str {
		match gltype {
			gl::DEBUG_TYPE_ERROR => "Error",
			gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behaviour",
			gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behaviour",
			gl::DEBUG_TYPE_PORTABILITY => "Portability",
			gl::DEBUG_TYPE_PERFORMANCE => "Performance",
			gl::DEBUG_TYPE_MARKER => "Marker",
			gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
			gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
			gl::DEBUG_TYPE_OTHER => "Other",
			_ => "Unknown",
		}
	}
	match severity {
		gl::DEBUG_SEVERITY_HIGH => {
			log::warn!(
				"OpenGL Message [HIGH SEVERITY | {} from {}] ({}): {}",
				type_str(gltype),
				source_str(source),
				id,
				unsafe { CStr::from_ptr(message) }.to_str().unwrap(),
			);
		}
		gl::DEBUG_SEVERITY_MEDIUM => {
			log::warn!(
				"OpenGL Message [MEDIUM SEVERITY] ({}): {}\nSource: {} Type: {}",
				type_str(gltype),
				source_str(source),
				id,
				unsafe { CStr::from_ptr(message) }.to_str().unwrap(),
			);
		}
		gl::DEBUG_SEVERITY_LOW => {
			log::info!(
				"OpenGL Message [LOW SEVERITY] ({}): {}\nSource: {} Type: {}",
				type_str(gltype),
				source_str(source),
				id,
				unsafe { CStr::from_ptr(message) }.to_str().unwrap(),
			);
		}
		gl::DEBUG_SEVERITY_NOTIFICATION => {
			log::info!(
				"OpenGL Message [NOTIFICATION] ({}): {}\nSource: {} Type: {}",
				type_str(gltype),
				source_str(source),
				id,
				unsafe { CStr::from_ptr(message) }.to_str().unwrap(),
			);
		}
		_ => {
			log::error!("unknown debug message");
		}
	}
}
