use glium::{texture::TextureCreationError, SwapBuffersError};
use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GfxError {
  #[error("{0}")]
  SwapBuffers(#[from] SwapBuffersError),
  #[error("{0}")]
  TextureCreation(#[from] TextureCreationError),
  #[error("{0}")]
  Image(#[from] ImageError),
  #[error("Texture {0} not found")]
  TextureNotFound(String),
  #[error("Sampler {0} not found")]
  SamplerNotFound(u16),
  #[error("No active camera to render with")]
  NoActiveCamera,
}
