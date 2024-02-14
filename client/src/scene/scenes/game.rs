use crate::{cmd::queue::CommandQueue, ecs::{components::{Camera, Renderable, Transform}, world::World}, error::ClientError, gfx::{color::Color, mesh::Mesh, renderer::Renderer, texture::Texture}, scene::scene::Scene};

/// The game scene.
pub struct GameScene;

impl Scene for GameScene {
  fn load(
    &mut self, 
    _command_queue: &mut CommandQueue,
    _renderer: &mut Renderer, 
    world: &mut World,
  ) -> Result<(), ClientError> {
    // Spawn "me".
    let me = world.spawn_entity((
      Transform::new([0.0, 0.0], [128.0, 128.0]),
      Renderable::new(Color::red(), Texture::none(), Mesh::square()),
      Camera::new([0.0, 0.0]),
    ));
    world.actives.set_camera(me);
    Ok(())
  }
  fn frame(
    &mut self,
    _command_queue: &mut CommandQueue,
    _renderer: &mut Renderer, 
    _world: &mut World,
  ) -> Result<(), ClientError> {
    Ok(())
  }
  fn unload(
    &mut self,
    _command_queue: &mut CommandQueue,
    _renderer: &mut Renderer,
    _world: &mut World,
  ) -> Result<(), ClientError> {
    Ok(())
  }
}