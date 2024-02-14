use hecs::Entity;
use crate::gfx::error::GfxError;

/// Active entities.
#[derive(Default)]
pub struct Actives {
  camera: Option<Entity>,
}

impl Actives {
  /// Get the active camera.
  pub fn camera(&self) -> Result<Entity, GfxError> {
    self.camera.ok_or(GfxError::NoActiveCamera)
  }
  /// Set the active camera.
  pub fn set_camera(&mut self, entity: Entity) {
    self.camera.replace(entity);
  }
}