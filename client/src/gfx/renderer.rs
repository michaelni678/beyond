use glium::{glutin::surface::WindowSurface, Display, Surface};
use crate::gfx::error::GfxError;

/// Renders stuff to the display.
pub struct Renderer {
  /// The GL context and facade.
  display: Display<WindowSurface>,
}

impl Renderer {
  /// Create a new renderer.
  pub fn new(display: Display<WindowSurface>) -> Self {
    Self {
      display: display,
    }
  }
  /// Execute the renderer.
  pub fn execute(&mut self) -> Result<(), GfxError> {
    // Get a frame and clear it.
    let mut frame = self.display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    // Execute.
    let result = (|| {
      Ok(())
    })();
    // Present and destroy the frame.
    frame.finish()?;
    // Return the execution results.
    result
  }
}