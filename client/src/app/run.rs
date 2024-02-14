use crate::{
  app::{
    events::{close_request, exit, frame, init},
    setup::window_builder,
  },
  errors::ClientError,
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
  // Run the event loop.
  event_loop.run(|event, elwt| {
    if let Err(error) = (|| -> Result<(), ClientError> {
      // Handle events.
      match event {
        // Application init event.
        Event::NewEvents(StartCause::Init) => init()?,
        // Application exit event.
        Event::LoopExiting => exit()?,
        // Request the next frame.
        Event::AboutToWait => window.request_redraw(),
        // Application window event.
        Event::WindowEvent { window_id, event } => {
          // Assert that the window ids are equivalent.
          assert_eq!(window.id(), window_id);
          // Handle application window events.
          match event {
            // Application close request event.
            WindowEvent::CloseRequested => close_request(elwt)?,
            // Frame event.
            WindowEvent::RedrawRequested => frame()?,
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
