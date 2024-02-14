use crate::{app::run::run_app, errors::ClientError, net::run::run_net};
use winit::event_loop::EventLoop;

mod app;
mod errors;
mod net;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
  // Create the event loop.
  let event_loop = EventLoop::new()?;
  // Spawn network task.
  let net_task = tokio::task::spawn(async move { run_net().await });
  // Block in place for application.
  tokio::task::block_in_place(|| run_app(event_loop))?;
  // Wait for the network task to finish.
  net_task.await??;
  Ok(())
}
