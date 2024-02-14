use winit::event_loop::EventLoopWindowTarget;

use crate::errors::ClientError;

/// App init event.
pub fn init() -> Result<(), ClientError> {
  Ok(())
}

/// App close request event.
pub fn close_request(elwt: &EventLoopWindowTarget<()>) -> Result<(), ClientError> {
  // Set to exit.
  elwt.exit();
  Ok(())
}

/// App exit event.
pub fn exit() -> Result<(), ClientError> {
  Ok(())
}
