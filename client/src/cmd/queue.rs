use std::collections::VecDeque;
use crate::{cmd::command::Command, ecs::world::World, error::ClientError, gfx::renderer::Renderer, scene::scene::Scenes};

/// A queue of commands.
#[derive(Default)]
pub struct CommandQueue {
  /// The queue of commands.
  queue: VecDeque<Box<dyn Command>>,
}

impl CommandQueue {
  /// Create a new command queue.
  pub fn new() -> Self {
    Self::default()
  }
  /// Push a command to the queue.
  pub fn enqueue(&mut self, command: impl Command + 'static) {
    self.queue.push_back(Box::new(command));
  }
  /// Execute the commands in the queue.
  pub fn execute(
    &mut self,
    renderer: &mut Renderer,
    scenes: &mut Scenes,
    world: &mut World,
  ) -> Result<(), ClientError> {
    while let Some(command) = self.queue.pop_front() {
      command.execute(self, renderer, scenes, world)?;
    }
    Ok(())
  }
}
