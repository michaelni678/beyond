use crate::{
  ecs::components::{Renderable, Transform},
  gfx::{color::Color, mesh::RawMesh, texture::Texture},
  math::{Point, Scale},
};

/// Render requests.
pub struct RenderRequest;

impl RenderRequest {
  /// A square (origin is at the top left).
  pub fn square(
    position: impl Into<Point>,
    scale: impl Into<Scale>,
    color: Color,
    texture: Texture,
  ) -> (Transform, Renderable) {
    (
      Transform::new(position, scale),
      Renderable::new(color, texture, RawMesh::square()),
    )
  }
  /// A point (origin is at the center).
  pub fn point(position: impl Into<Point>, color: Color) -> (Transform, Renderable) {
    RenderRequest::circle16(position, [8.0, 8.0], color)
  }
  /// A 16-point circle (origin is at the center).
  pub fn circle16(
    position: impl Into<Point>,
    scale: impl Into<Scale>,
    color: Color,
  ) -> (Transform, Renderable) {
    (
      Transform::new(position, scale),
      Renderable::new(color, Texture::none(), RawMesh::circle16()),
    )
  }
}
