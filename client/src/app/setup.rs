use winit::window::WindowBuilder;
use crate::scene::{scene::Scenes, scenes::game::GameScene};

/// The window builder of the application.
pub fn window_builder() -> WindowBuilder {
  WindowBuilder::new().with_title("Beyond")
}

/// Scene registry.
pub fn scenes() -> Scenes {
  Scenes::new().register(GameScene)
}