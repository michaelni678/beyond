use crate::{
  ecs::{
    components::{Camera, Renderable, Transform},
    world::World,
  },
  error::ClientError,
  gfx::{
    error::GfxError,
    pipeline::{GfxPipeline, GfxPipelineAttributes},
    programs::Programs,
    texture::Textures,
  },
};
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
  /// The program manager.
  programs: Programs,
  /// Oneshot render requests.
  oneshot_requests: Vec<(Transform, Renderable)>,
}

impl Renderer {
  /// Create a new renderer.
  pub fn new(display: Display<WindowSurface>) -> Result<Self, GfxError> {
    let textures = Textures::new(&display)?;
    let programs = Programs::new(&display)?;
    Ok(Self {
      display: display,
      pipelines: FxHashMap::default(),
      textures: textures,
      programs: programs,
      oneshot_requests: Vec::new(),
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
  /// Add a new oneshot render request.
  pub fn add_oneshot_request(&mut self, request: (Transform, Renderable)) {
    self.oneshot_requests.push(request);
  }
  /// Execute the renderer.
  pub fn execute(&mut self, world: &mut World) -> Result<(), ClientError> {
    // Get a frame and clear it.
    let mut frame = self.display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    // Execute.
    let result = (|| {
      // Get the projection matrix.
      let projection = {
        // Get the active camera and inspect.
        let active_camera = world.actives.camera()?;
        let (transform, camera) = world.standard_inspect::<(&Transform, &Camera)>(active_camera)?;
        let fbd = self.display.get_framebuffer_dimensions();
        camera.projection(fbd, transform.position)
      };
      // Query the renderables.
      let query = world
        .standard_query::<(&Transform, &mut Renderable)>()
        .into_iter()
        .map(|(_, data)| data);
      let oneshots = self.oneshot_requests.iter_mut().map(|(t, r)| (&*t, r));
      let chain = query.into_iter().chain(oneshots);
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
          let pipeline = GfxPipeline::new(&self.display, &attrs, None)?;
          self.pipelines.entry(attrs).or_insert(pipeline)
        };
        // Write to the pipeline.
        pipeline.write(
          &mut frame,
          &self.programs,
          &self.textures,
          &projection,
          transform.position,
          transform.scale,
          renderable.color.into(),
          texture_info,
          &renderable.mesh,
        )?;
      }
      // Loop through the pipelines, flushing them.
      for pipeline in self.pipelines.values_mut() {
        pipeline.flush(&mut frame, &self.programs, &self.textures, projection)?;
      }
      Ok(())
    })();
    // Present and destroy the frame.
    frame.finish().map_err(GfxError::from)?;
    // Return the execution results.
    result
  }
}
