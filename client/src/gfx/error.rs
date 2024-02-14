use glium::{index, texture::TextureCreationError, vertex, DrawError, ProgramCreationError, SwapBuffersError};
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
  #[error("{0}")]
  VBOCreation(#[from] vertex::BufferCreationError),
  #[error("{0}")]
  IBOCreation(#[from] index::BufferCreationError),
  #[error("Failed to slice buffer")]
  BufferSlice,
  #[error("{0}")]
  ProgramCreation(#[from] ProgramCreationError),
  #[error("{0}")]
  Draw(#[from] DrawError),
}
