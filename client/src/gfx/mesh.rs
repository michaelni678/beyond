use crate::{math::Point, misc::flag::Flag};

/// A mesh.
pub struct Mesh {
  /// The vertices of the mesh.
  vertices: Flag<Box<[Point]>>,
  /// The indices of the mesh. Must be recalculated if
  /// `vertices` is changed.
  indices: Box<[u16]>,
}

impl Mesh {
  /// Create a new mesh.
  pub fn new(vertices: Vec<Point>, indices: Vec<u16>) -> Self {
    let vertices = vertices.into_boxed_slice();
    Self {
      vertices: Flag::new_clean(vertices),
      indices: indices.into_boxed_slice(),
    }
  }
  /// Get the indices of the mesh.
  pub fn vertices(&self) -> &[Point] {
    self.vertices.get().as_ref()
  }
  /// Get the indices of the mesh.
  pub fn indices(&mut self) -> Box<[u16]> {
    // If the vertices are dirty, the indices must be recalculated
    // by triangulating the vertices.
    if self.vertices.is_dirty() {
      // TODO:
      // At the moment, vertices cannot be changed (otherwise this will
      // panic).
      todo!();
      // self.vertices.clean();
    }
    self.indices.clone()
  }
  /// A unit square mesh.
  pub fn square() -> Self {
    Self::new(
      vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(0.0, 1.0),
      ],
      vec![0, 2, 1, 0, 3, 2],
    )
  }
}
