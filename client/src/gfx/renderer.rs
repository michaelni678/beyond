use crate::{ecs::{components::{Renderable, Transform}, world::World}, gfx::{error::GfxError, pipeline::{GfxPipeline, GfxPipelineAttributes}, texture::Textures}};
use glium::{glutin::surface::WindowSurface, Display, Surface};
use rustc_hash::FxHashMap;

/// Renders stuff to the display.
pub struct Renderer {
  /// The GL context and facade.
  display: Display<WindowSurface>,
  /// The pipelines of the renderer.
  pipelines: FxHashMap<GfxPipelineAttributes, GfxPipeline>,
  /// The texture manager.
  pub textures: Textures,
  /// Render requests.
  render_requests: Vec<(Transform, Renderable)>,
}

impl Renderer {
  /// Create a new renderer.
  pub fn new(display: Display<WindowSurface>) -> Result<Self, GfxError> {
    let textures = Textures::new(&display)?;
    Ok(Self { 
      display: display,
      pipelines: FxHashMap::default(),
      textures: textures,
      render_requests: Vec::new(),
    })
  }
  /// Add a new sampler.
  /// Returns it's id.
  pub fn add_sampler(
    &mut self,
    bytes: impl AsRef<[u8]>,
    info: impl IntoIterator<Item = (impl ToString, Vec<[f32; 2]>)>,
  ) -> Result<u16, GfxError> {
    self.textures.add_sampler(&self.display, bytes, info)
  }
  /// Execute the renderer.
  pub fn execute(&mut self, world: &mut World) -> Result<(), GfxError> {
    // Get a frame and clear it.
    let mut frame = self.display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    // Execute.
    let result = (|| {
      // Query the renderables.
      let query = world
        .standard_query::<(&Transform, &mut Renderable)>()
        .into_iter()
        .map(|(_, data)| data);
      let requests = self.render_requests.iter_mut().map(|(t, r)| (&*t, r));
      let chain = query.into_iter().chain(requests);
      for (transform, renderable) in chain {
        // Get the texture information of the renderable.
        let texture_info = self.textures.get_texture_info(renderable.texture.get())?;
        // Determine the pipeline attributes required to render the renderable.
        let attrs = GfxPipelineAttributes::new(renderable.mesh.indices(), texture_info.sampler_id);
        // Attempt to get a pipeline that matches the attributes. 
        // If one doesn't exist, create a new valid pipeline.
        let pipeline = if let Some(pipeline) = self.pipelines.get_mut(&attrs) {
          pipeline
        } else {
          let pipeline = GfxPipeline::new()?;
          self.pipelines.entry(attrs).or_insert(pipeline)
        };
      }
      Ok(())
    })();
    // Present and destroy the frame.
    frame.finish()?;
    // Return the execution results.
    result
  }
}
