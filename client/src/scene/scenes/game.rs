use crate::{
  cmd::queue::CommandQueue,
  ecs::{
    components::{Camera, Renderable, Transform},
    world::World,
  },
  error::ClientError,
  gfx::{
    color::Color, mesh::RawMesh, renderer::Renderer, request::RenderRequest, texture::Texture,
  },
  scene::scene::Scene,
};

/// The game scene.
pub struct GameScene;

impl Scene for GameScene {
  fn load(
    &mut self,
    _command_queue: &mut CommandQueue,
    _renderer: &mut Renderer,
    world: &mut World,
  ) -> Result<(), ClientError> {
    // Spawn "me" and set the active camera to it.
    let me = world.spawn_entity((
      Transform::new([0.0, 0.0], [128.0, 128.0]),
      Renderable::new(
        Color::none(),
        Texture::regular("Standalone"),
        RawMesh::square(),
      ),
      Camera::new([0.0, 0.0]),
    ));
    world.actives.set_camera(me);
    Ok(())
  }
  fn frame(
    &mut self,
    _command_queue: &mut CommandQueue,
    renderer: &mut Renderer,
    _world: &mut World,
  ) -> Result<(), ClientError> {
    // Add oneshot render requests.
    renderer.add_oneshot_request(RenderRequest::square(
      [0.0, 128.0],
      [128.0, 128.0],
      Color::red(),
      Texture::none(),
    ));
    renderer.add_oneshot_request(RenderRequest::circle16(
      [128.0, 0.0],
      [128.0, 128.0],
      Color::blue(),
    ));
    renderer.add_oneshot_request(RenderRequest::point([-128.0, 0.0], Color::blue()));
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
