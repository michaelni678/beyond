use crate::{cmd::{command::commands::LoadScene, queue::CommandQueue}, ecs::world::World, error::ClientError, gfx::renderer::Renderer, include_wrt_manifest, scene::{scene::Scenes, scenes::game::GameScene}};
use winit::event_loop::EventLoopWindowTarget;

/// App init event.
pub fn init(
  command_queue: &mut CommandQueue,
  renderer: &mut Renderer,
  _world: &mut World,
) -> Result<(), ClientError> {
  command_queue.enqueue(LoadScene::<GameScene>);
  renderer.add_sampler(
    include_wrt_manifest!("/res/textures/standalone.png"),
    [(
      "Standalone",
      vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    )],
  )?;
  Ok(())
}

/// App exit event.
pub fn exit(
  _command_queue: &mut CommandQueue,
  _renderer: &mut Renderer,
  _world: &mut World,
) -> Result<(), ClientError> {
  Ok(())
}

/// Frame event.
pub fn frame(
  command_queue: &mut CommandQueue,
  renderer: &mut Renderer,
  scenes: &mut Scenes,
  world: &mut World,
) -> Result<(), ClientError> {
  // Execute the command queue.
  command_queue.execute(renderer, scenes, world)?;
  // Get the scene and execute the frame.
  let scene = scenes.loaded()?;
  scene.frame(command_queue, renderer, world)?;
  // Execute the renderer.
  renderer.execute(world)?;
  Ok(())
}

/// App close request event.
pub fn close_request(
  elwt: &EventLoopWindowTarget<()>,
  _command_queue: &mut CommandQueue,
  _renderer: &mut Renderer,
  _world: &mut World,
) -> Result<(), ClientError> {
  // Exit event loop.
  elwt.exit();
  Ok(())
}
