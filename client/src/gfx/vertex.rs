use glium::implement_vertex;

/// The vertex of a pipeline.
#[derive(Copy, Clone, Default)]
pub struct Vertex {
  pub position: [f32; 2],
  pub color: [f32; 4],
  pub texture_coord: [f32; 2],
}

implement_vertex!(Vertex, position, color, texture_coord);