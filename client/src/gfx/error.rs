use glium::SwapBuffersError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GfxError {
  #[error("{0}")]
  SwapBuffers(#[from] SwapBuffersError),
}
