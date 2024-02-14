use crate::gfx::error::GfxError;

/// Pipeline for graphics.
pub struct GfxPipeline {

}

impl GfxPipeline {
  /// Create a new graphics pipeline.
  pub fn new() -> Result<Self, GfxError> {
    Ok(Self {

    })
  }
}

/// Graphics pipeline attributes.
#[derive(PartialEq, Eq, Hash)]
pub struct GfxPipelineAttributes {
  index_pattern: Box<[u16]>,
  sampler_id: u16,
}

impl GfxPipelineAttributes {
  /// Create a new attributes object.
  pub fn new(index_pattern: Box<[u16]>, sampler_id: u16) -> Self {
    Self {
      index_pattern: index_pattern,
      sampler_id: sampler_id,
    }
  }
}