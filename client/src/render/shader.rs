use crate::render::RenderError;

use std::ffi::CString;
use std::path::Path;

pub enum ShaderKind {
	Vertex,
	Fragement,
	#[allow(dead_code)]
	Geometry,
}

impl ShaderKind {
	pub fn value(&self) -> u32 {
		match self {
			ShaderKind::Vertex => gl::VERTEX_SHADER,
			ShaderKind::Fragement => gl::FRAGMENT_SHADER,
			ShaderKind::Geometry => gl::GEOMETRY_SHADER,
		}
	}
}

pub struct Shader {
	id: gl::types::GLuint,
}

impl Shader {
	pub fn from_str(src: &str, kind: ShaderKind) -> Result<Shader, RenderError> {
		// Compile Shader
		let cstr = CString::new(src).unwrap();
		let id = unsafe {
			let id = gl::CreateShader(kind.value());
			gl::ShaderSource(id, 1, &cstr.as_ptr(), std::ptr::null());
			gl::CompileShader(id);
			id
		};
		// Check for error and get as string
		let mut success: gl::types::GLint = 1;
		unsafe {
			gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
			if success == 0 {
				// Get the length of the log
				let mut len: gl::types::GLint = 0;
				gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
				// Create a buffer for it
				let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
				buffer.extend([b' '].iter().cycle().take(len as usize));
				let error = CString::from_vec_unchecked(buffer);
				// Give it to OpenGL
				gl::GetShaderInfoLog(
					id,
					len,
					std::ptr::null_mut(),
					error.as_ptr() as *mut gl::types::GLchar,
				);
				return Err(RenderError::Shader(error.into_string().unwrap()));
			}
		}
		Ok(Shader { id })
	}

	pub fn from_file<P: AsRef<Path>>(path: P, kind: ShaderKind) -> Result<Shader, RenderError> {
		let source = std::fs::read_to_string(path.as_ref());
		match source {
			Ok(s) => Self::from_str(&s, kind),
			Err(e) => Err(RenderError::Shader(format!("io error: {:?}", e))), // stringly typed error ew
		}
	}

	pub fn id(&self) -> gl::types::GLuint {
		self.id
	}
}

impl core::ops::Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.id);
		}
	}
}

pub struct Program {
	id: gl::types::GLuint,
}

impl Program {
	pub fn from_shaders(shaders: &[Shader]) -> Result<Program, RenderError> {
		let id = unsafe { gl::CreateProgram() };
		unsafe {
			for shader in shaders {
				gl::AttachShader(id, shader.id());
			}
			gl::LinkProgram(id);
			for shader in shaders {
				gl::DetachShader(id, shader.id());
			}
		}
		// Check for error and get as string
		let mut success: gl::types::GLint = 1;
		unsafe {
			gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
			if success == 0 {
				// Get the length of the log
				let mut len: gl::types::GLint = 0;
				gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
				// Create a buffer for it
				let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
				buffer.extend([b' '].iter().cycle().take(len as usize));
				let error = CString::from_vec_unchecked(buffer);
				// Give it to OpenGL
				gl::GetProgramInfoLog(
					id,
					len,
					std::ptr::null_mut(),
					error.as_ptr() as *mut gl::types::GLchar,
				);
				return Err(RenderError::Shader(error.into_string().unwrap()));
			}
		}
		Ok(Program { id })
	}
	pub fn from_vert_and_frag(vert: &str, frag: &str) -> Result<Program, RenderError> {
		let vert = Shader::from_str(vert, ShaderKind::Vertex)?;
		let frag = Shader::from_str(frag, ShaderKind::Fragement)?;
		Self::from_shaders(&[vert, frag])
	}

	pub fn bind(&self) {
		unsafe { gl::UseProgram(self.id) }
	}

	pub fn set_uniform_int(&self, name: &str, val: i32) {
		let cstr = CString::new(name).unwrap();
		unsafe {
			gl::ProgramUniform1i(
				self.id,
				gl::GetUniformLocation(self.id, cstr.as_ptr() as *const gl::types::GLchar),
				val,
			);
		}
	}

	pub fn set_uniform_mat4(&self, name: &str, mat: ultraviolet::mat::Mat4) {
		let cstr = CString::new(name).unwrap();
		unsafe {
			gl::ProgramUniformMatrix4fv(
				self.id,
				gl::GetUniformLocation(self.id, cstr.as_ptr() as *const gl::types::GLchar),
				1,
				gl::FALSE,
				mat.as_ptr(),
			);
		}
	}
}

impl core::ops::Drop for Program {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}
