use thiserror::Error;
use tokio::task::JoinError;
use winit::error::EventLoopError;

#[derive(Error, Debug)]
pub enum ClientError {
  #[error("{0}")]
  JoinError(#[from] JoinError),
  #[error("{0}")]
  EventLoop(#[from] EventLoopError),
}
