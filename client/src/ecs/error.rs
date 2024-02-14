use hecs::Entity;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EcsError {
  #[error("Entity {0:?} was not found")]
  EntityNotFound(Entity),
  #[error("World inspect unsatisfied")]
  UnsatisfiedInspect,
}
