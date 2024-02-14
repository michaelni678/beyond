use glium::{glutin::surface::WindowSurface, index::PrimitiveType, uniform, Blend, Display, DrawParameters, Frame, IndexBuffer, Surface, VertexBuffer};
use crate::{gfx::{error::GfxError, mesh::Mesh, programs::Programs, texture::{TextureInfo, Textures}, vertex::Vertex}, math::{Point, Scale}};

/// The default flush threshold for a pipeline.
const DEFAULT_FLUSH_THRESHOLD: usize = 64;

/// Pipeline for graphics.
pub struct GfxPipeline {
  vertex_data: Box<[Vertex]>,
  vertex_buffer: VertexBuffer<Vertex>,
  index_buffer: IndexBuffer<u16>,
  vertices_per_mesh: usize,
  indices_per_mesh: usize,
  len: usize,
  flush_threshold: usize,
  sampler_id: u16,
}

impl GfxPipeline {
  /// Create a new graphics pipeline.
  pub fn new(
    display: &Display<WindowSurface>,
    attributes: &GfxPipelineAttributes,
    flush_threshold: Option<usize>,
  ) -> Result<Self, GfxError> {
    // Get the flush threshold.
    let flush_threshold = flush_threshold.unwrap_or(DEFAULT_FLUSH_THRESHOLD);
    // Determine the number of vertices of the mesh.
    let vertices_per_mesh = {
      let mut index_pattern = attributes.index_pattern.to_vec();
      index_pattern.sort();
      index_pattern.dedup();
      index_pattern.len()
    };
    // Generate the index data.
    let indices_per_mesh = attributes.index_pattern.len();
    let index_data = {
      let mut repeated = attributes.index_pattern.repeat(flush_threshold);
      let chunks = repeated.chunks_mut(indices_per_mesh);
      for (i, chunk) in chunks.enumerate() {
        for index in chunk {
          *index += (vertices_per_mesh * i) as u16;
        }
      }
      repeated
    };
    // Return the pipeline.
    let buffer_len = flush_threshold * vertices_per_mesh;
    Ok(Self {
      vertex_data: vec![Vertex::default(); buffer_len].into_boxed_slice(),
      vertex_buffer: VertexBuffer::empty_dynamic(display, buffer_len)?,
      index_buffer: IndexBuffer::immutable(display, PrimitiveType::TrianglesList, &index_data)?,
      vertices_per_mesh: vertices_per_mesh,
      indices_per_mesh: indices_per_mesh,
      len: 0,
      flush_threshold: flush_threshold,
      sampler_id: attributes.sampler_id,
    })
  }
  /// Write to the pipeline.
  pub fn write(
    &mut self,
    frame: &mut Frame,
    programs: &Programs,
    textures: &Textures,
    projection: &[[f32; 4]; 4],
    position: Point,
    scale: Scale,
    color: [f32; 4],
    texture_info: &TextureInfo,
    mesh: &Mesh,
  ) -> Result<(), GfxError> {
    // Check if a flush is necessary first.
    if self.len >= self.flush_threshold {
      self.flush(frame, programs, textures, *projection)?;
    }
    // Loop through the vertices.
    for (i, &vertex) in mesh.vertices().iter().enumerate() {
      // Calculate vertex information.
      let vertex_position = {
        let sv = scale * vertex;
        Point::new(position.x + sv.x, position.y + sv.y)
      };
      // Cache the vertex to write to.
      let vertex = &mut self.vertex_data[self.len * self.vertices_per_mesh + i];
      // Write to the cached vertex.
      vertex.position = vertex_position.into();
      vertex.color = color;
      vertex.texture_coord = texture_info.texture_coords[i];
    }
    self.len += 1;
    Ok(())
  }
  /// Flush the pipeline.
  pub fn flush(
    &mut self,
    frame: &mut Frame,
    programs: &Programs,
    textures: &Textures,
    projection: [[f32; 4]; 4],
  ) -> Result<(), GfxError> {
    // Check if there is anything to flush.
    if self.len > 0 {
      // Write the vertex data to the vertex buffer.
      self.vertex_buffer.write(&self.vertex_data);
      // Slice the buffers.
      let vertex_buffer_slice = self
        .vertex_buffer
        .slice(0..self.len * self.vertices_per_mesh)
        .ok_or(GfxError::BufferSlice)?;
      let index_buffer_slice = self
        .index_buffer
        .slice(0..self.len * self.indices_per_mesh)
        .ok_or(GfxError::BufferSlice)?;
      // Get the sampler.
      let sampler = textures.get_sampler(self.sampler_id)?;
      // Draw the frame.
      frame.draw(
        vertex_buffer_slice,
        index_buffer_slice,
        &programs.basic,
        &uniform! {
          u_projection: projection,
          u_sampler: sampler,
        },
        &DrawParameters {
          blend: Blend::alpha_blending(),
          ..Default::default()
        },
      )?;
      // Reset the length.
      self.len = 0;
    }
    Ok(())
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