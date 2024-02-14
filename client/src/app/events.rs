use winit::event_loop::EventLoopWindowTarget;
use crate::{error::ClientError, gfx::renderer::Renderer};

/// App init event.
pub fn init(_renderer: &mut Renderer) -> Result<(), ClientError> {
  Ok(())
}

/// App exit event.
pub fn exit(_renderer: &mut Renderer) -> Result<(), ClientError> {
  Ok(())
}

/// Frame event.
pub fn frame(renderer: &mut Renderer) -> Result<(), ClientError> {
  // Execute the renderer.
  renderer.execute()?;
  Ok(())
}

/// App close request event.
pub fn close_request(
  elwt: &EventLoopWindowTarget<()>,
  _renderer: &mut Renderer,
  ) -> Result<(), ClientError> {
  // Exit event loop.
  elwt.exit();
  Ok(())
}