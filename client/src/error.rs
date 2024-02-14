use crate::{ecs::error::EcsError, gfx::error::GfxError, net::error::NetError, scene::error::SceneError};
use thiserror::Error;
use tokio::task::JoinError;
use winit::error::EventLoopError;

#[derive(Error, Debug)]
pub enum ClientError {
  #[error("{0}")]
  JoinError(#[from] JoinError),
  #[error("{0}")]
  EventLoop(#[from] EventLoopError),
  #[error("{0}")]
  Gfx(#[from] GfxError),
  #[error("{0}")]
  Scene(#[from] SceneError),
  #[error("{0}")]
  Net(#[from] NetError),
  #[error("{0}")]
  Ecs(#[from] EcsError),
}
