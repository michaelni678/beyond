use crate::{
  cmd::queue::CommandQueue, ecs::world::World, error::ClientError, gfx::renderer::Renderer,
  scene::scene::Scenes,
};

/// Defines a command.
pub trait Command {
  /// Execute the command.
  fn execute(
    self: Box<Self>,
    command_queue: &mut CommandQueue,
    renderer: &mut Renderer,
    scenes: &mut Scenes,
    world: &mut World,
  ) -> Result<(), ClientError>;
}

/// Command variants.
pub mod commands {
  use crate::{
    cmd::{command::Command, queue::CommandQueue},
    ecs::world::World,
    error::ClientError,
    gfx::renderer::Renderer,
    scene::scene::{Scene, Scenes},
  };

  /// Load a scene.
  #[ghost::phantom]
  pub struct LoadScene<S: Scene>;

  impl<S: Scene> Command for LoadScene<S> {
    fn execute(
      self: Box<Self>,
      command_queue: &mut CommandQueue,
      renderer: &mut Renderer,
      scenes: &mut Scenes,
      world: &mut World,
    ) -> Result<(), ClientError> {
      scenes.load::<S>(command_queue, renderer, world)?;
      Ok(())
    }
  }
}
