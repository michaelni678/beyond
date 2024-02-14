use winit::window::WindowBuilder;

/// The window builder of the application.
pub fn window_builder() -> WindowBuilder {
  WindowBuilder::new().with_title("Beyond")
}
