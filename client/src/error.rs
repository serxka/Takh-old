use crate::render::RenderError;
use image::error::ImageError;

#[derive(Debug)]
pub enum Error {
	BackendError(Box<dyn std::fmt::Debug>),
	RenderError(RenderError),
	AssetError(Box<dyn std::fmt::Debug>),
}

impl From<RenderError> for Error {
	fn from(e: RenderError) -> Error {
		Error::RenderError(e)
	}
}

impl From<ImageError> for Error {
	fn from(e: ImageError) -> Error {
		Error::AssetError(Box::new(e))
	}
}
