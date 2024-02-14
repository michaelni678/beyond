use glium::{glutin::surface::WindowSurface, Display, Program};

use crate::gfx::error::GfxError;

/// A basic vertex shader.
const BASIC_VS: &str = "
  #version 330 core

  layout (location = 0) in vec2 position;
  layout (location = 1) in vec4 color;
  layout (location = 2) in vec2 texture_coord;

  out vec4 v_color;
  out vec2 v_texture_coord;

  uniform mat4 u_projection;

  void main() {
    gl_Position = u_projection * vec4(position, 0.0, 1.0);
    v_color = color;
    v_texture_coord = texture_coord;
  }
";

/// A basic fragment shader.
const BASIC_FS: &str = "
  #version 330 core

  in vec4 v_color;
  in vec2 v_texture_coord;

  out vec4 f_color;

  uniform sampler2D u_sampler;

  void main() {
    f_color = v_color * texture2D(u_sampler, v_texture_coord);
  }
";

/// Manages programs.
pub struct Programs {
  pub basic: Program,
}

impl Programs {
  /// Create a new programs manager.
  pub fn new(display: &Display<WindowSurface>) -> Result<Self, GfxError> {
    Ok(Self {
      basic: Program::from_source(display, BASIC_VS, BASIC_FS, None)?,
    })
  }
}