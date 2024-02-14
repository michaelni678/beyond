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
}

impl From<RawMesh> for Mesh {
  fn from(raw: RawMesh) -> Self {
    Self::new(raw.vertices, raw.indices)
  }
}

/// A raw mesh.
pub struct RawMesh {
  vertices: Vec<Point>,
  indices: Vec<u16>,
}

impl RawMesh {
  /// Create a square raw mesh.
  pub fn square() -> Self {
    Self {
      vertices: vec![
        Point::new(-0.5, -0.5),
        Point::new(0.5, -0.5),
        Point::new(0.5, 0.5),
        Point::new(-0.5, 0.5),
      ],
      indices: vec![0, 2, 1, 0, 3, 2],
    }
  }
  /// Create a 16-point circle raw mesh.
  pub fn circle16() -> Self {
    Self {
      vertices: vec![
        Point::new(0.0, 0.5),           // Top point.
        Point::new(0.19134, 0.46194),   // 22.5 degrees.
        Point::new(0.35355, 0.35355),   // 45 degrees.
        Point::new(0.46194, 0.19134),   // 67.5 degrees.
        Point::new(0.5, 0.0),           // Right point.
        Point::new(0.46194, -0.19134),  // 112.5 degrees.
        Point::new(0.35355, -0.35355),  // 135 degrees.
        Point::new(0.19134, -0.46194),  // 157.5 degrees.
        Point::new(0.0, -0.5),          // Bottom point.
        Point::new(-0.19134, -0.46194), // 202.5 degrees.
        Point::new(-0.35355, -0.35355), // 225 degrees.
        Point::new(-0.46194, -0.19134), // 247.5 degrees.
        Point::new(-0.5, 0.0),          // Left point.
        Point::new(-0.46194, 0.19134),  // 292.5 degrees.
        Point::new(-0.35355, 0.35355),  // 315 degrees.
        Point::new(-0.19134, 0.46194),  // 337.5 degrees.
      ],
      indices: vec![
        0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 7, 0, 7, 8, 0, 8, 9, 0, 9, 10, 0, 10,
        11, 0, 11, 12, 0, 12, 13, 0, 13, 14, 0, 14, 15, 0, 15, 1,
      ],
    }
  }
}
