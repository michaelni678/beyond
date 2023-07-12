use crate as beyond_tap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AtlasInfo {
  pub w: u32,
  pub h: u32,
  pub texture_infos: Vec<beyond_tap::texture_info::TextureInfo>,
}