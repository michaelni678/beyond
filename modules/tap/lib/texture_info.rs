#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TextureInfo {
  pub label: String,
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
  pub is_rotated: bool,
}