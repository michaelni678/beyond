use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneError {
  #[error("Scene not found")]
  SceneNotFound,
}
