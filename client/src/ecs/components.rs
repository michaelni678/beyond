use crate::{gfx::{color::Color, mesh::Mesh, texture::Texture}, math::{Matrix4, Point, Scale}};

/// The position and scale of an entity.
pub struct Transform {
  pub position: Point,
  pub scale: Scale,
}

impl Transform {
  /// Create a new transform component.
  pub fn new(position: impl Into<Point>, scale: impl Into<Scale>) -> Self {
    Self {
      position: position.into(),
      scale: scale.into(),
    }
  }
}

/// Allows an entity to be rendered.
pub struct Renderable {
  pub color: Color,
  pub texture: Texture,
  pub mesh: Mesh,
}

impl Renderable {
  /// Create a new renderable component.
  pub fn new(color: Color, texture: Texture, mesh: Mesh) -> Self {
    Self {
      color: color,
      texture: texture,
      mesh: mesh,
    }
  }
}

/// The camera component.
pub struct Camera {
  pub offset: [f32; 2],
}

impl Camera {
  /// Create a new camera.
  pub fn new(offset: [f32; 2]) -> Self {
    Self { offset: offset }
  }
  /// Get the projection matrix.
  /// `fbd` is the frame buffer dimensions.
  /// `position` is the position of the entity that holds the camera.
  pub fn projection(&self, fbd: (u32, u32), position: Point) -> [[f32; 4]; 4] {
    let left = position.x - (fbd.0 / 2) as f32 + self.offset[0];
    let right = position.x + (fbd.0 / 2) as f32 + self.offset[0];
    let bottom = position.y + (fbd.1 / 2) as f32 + self.offset[1];
    let top = position.y - (fbd.1 / 2) as f32 + self.offset[1];
    Matrix4::new_orthographic(left, right, bottom, top, -1.0, 1.0).into()
  }
}