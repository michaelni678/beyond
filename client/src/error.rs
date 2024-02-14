use crate::{gfx::error::GfxError, net::error::NetError};
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
  Net(#[from] NetError),
}
