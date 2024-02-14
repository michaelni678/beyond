use crate::{
  app::{
    events::{close_request, exit, frame, init},
    setup::{scenes, window_builder},
  }, cmd::queue::CommandQueue, ecs::world::World, error::ClientError, gfx::renderer::Renderer
};
use glium::backend::glutin::SimpleWindowBuilder;
use winit::{
  event::{Event, StartCause, WindowEvent},
  event_loop::EventLoop,
};

/// Runs the app.
pub fn run_app(event_loop: EventLoop<()>) -> Result<(), ClientError> {
  // Create the window.
  let (window, display) = {
    let wb = window_builder();
    SimpleWindowBuilder::new()
      .set_window_builder(wb)
      .build(&event_loop)
  };
  // Create the command queue.
  let mut command_queue = CommandQueue::new();
  // Create the renderer.
  let mut renderer = Renderer::new(display)?;
  // Create the scene manager.
  let mut scenes = scenes();
  // Create the world.
  let mut world = World::new();
  // Run the event loop.
  event_loop.run(|event, elwt| {
    if let Err(error) = (|| -> Result<(), ClientError> {
      // Handle events.
      match event {
        // Application init event.
        Event::NewEvents(StartCause::Init) => init(&mut command_queue, &mut renderer, &mut world)?,
        // Application exit event.
        Event::LoopExiting => exit(&mut command_queue, &mut renderer, &mut world)?,
        // Request the next frame.
        Event::AboutToWait => window.request_redraw(),
        // Application window event.
        Event::WindowEvent { window_id, event } => {
          // Assert that the window ids are equivalent.
          assert_eq!(window.id(), window_id);
          // Handle application window events.
          match event {
            // Application close request event.
            WindowEvent::CloseRequested => close_request(elwt, &mut command_queue, &mut renderer, &mut world)?,
            // Frame event.
            WindowEvent::RedrawRequested => frame(&mut command_queue, &mut renderer, &mut scenes, &mut world)?,
            // Ignore other window events.
            _ => (),
          }
        },
        // Ignore other events.
        _ => (),
      }
      Ok(())
    })() {
      eprintln!("{}", error);
    }
  })?;
  Ok(())
}
