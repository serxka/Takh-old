use crate::render::RenderError;

#[derive(Debug)]
pub enum Error {
	BackendError(Box<dyn std::fmt::Debug>),
	RenderError(RenderError),
}

impl From<RenderError> for Error {
	fn from(e: RenderError) -> Error {
		Error::RenderError(e)
	}
}
