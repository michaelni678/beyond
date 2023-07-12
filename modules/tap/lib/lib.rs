use crate as beyond_tap;

use texture_packer::texture::Texture;

pub mod json;
pub mod texture_info;
pub mod atlas_info;

pub fn run(tap_config_file_path: std::path::PathBuf) -> anyhow::Result<()> {
  // The outfile prefix. This is used later to name the atlas files
  let outfile_pfx = {
    let stem = tap_config_file_path.file_stem().ok_or_else(|| anyhow::anyhow!("failed to get file stem"))?;
    stem.to_string_lossy().to_string()
  };
  // Read the tap config
  let tap_config = beyond_tap::json::read_json(tap_config_file_path)?;
  // Create the output dir
  let out_dir = beyond_tap::json::get_field!(tap_config, "out_dir", as_str)?;
  std::fs::create_dir_all(out_dir.clone())?;
  // Get the actual packer config part of the tap config
  let tap_packer_config = beyond_tap::json::get_field!(tap_config, "packer_config", as_object)?;
  // Create the texture packer library config
  let max_width = beyond_tap::json::get_field!(tap_packer_config, "max_width", as_u64)? as u32;
  let max_height = beyond_tap::json::get_field!(tap_packer_config, "max_height", as_u64)? as u32;
  let packer_lib_config = texture_packer::TexturePackerConfig {
    max_width: max_width,
    max_height: max_height,
    allow_rotation: beyond_tap::json::get_field!(tap_packer_config, "allow_rotation", as_bool)?,
    // There may be some other settings. Just set them to the defaults.
    ..Default::default()
  };
  // Get the input directories and accepted extensions
  let in_dirs = beyond_tap::json::get_field!(tap_config, "in_dirs", as_array)?;
  let in_exts = beyond_tap::json::get_field!(tap_config, "in_exts", as_array)?;
  // Collect the texture paths by looping through the directory and extensions, building the required glob patterns
  // For example, if in_dirs = ["dir1", "dir2"] and in_exts = [".png", ".jpg"] the globs will be ["dir1/*.png", "dir1/*.jpg", "dir2/*.png", "dir2/*.jpg"]
  let mut texture_paths = Vec::new();
  for in_dir in in_dirs {
    if let Some(in_dir) = in_dir.as_str() {
      for in_ext in in_exts {
        if let Some(in_ext) = in_ext.as_str() {
          let pattern = format!("{}/*{}", in_dir, in_ext);
          texture_paths.extend(glob::glob(&pattern)?.filter_map(Result::ok));
        }
      }
    }
  }
  // Create the actual packer
  let mut packer = texture_packer::MultiTexturePacker::new_skyline(packer_lib_config);
  println!("Packing textures into atlas(es) of maximum size {}x{}", max_width, max_height);
  // Loop through the paths and pack
  for texture_path in texture_paths.iter() {
    // Import the texture
    let texture = texture_packer::importer::ImageImporter::import_from_file(&texture_path).map_err(|err| anyhow::anyhow!("{}", err))?;
    // Set the label of the texture
    let label = {
      let stem = texture_path.file_stem().ok_or_else(|| anyhow::anyhow!("failed to get file stem"))?;
      stem.to_string_lossy().to_string()
    };
    // Pack
    println!("\tPacking {}", label);
    packer.pack_own(label, texture).map_err(|err| anyhow::anyhow!("{:?}", err))?;
  }
  // Iterate the packed pages
  for (i, page) in packer.get_pages().iter().enumerate() {
    // Get the texture infos
    let mut packed_textures = Vec::new();
    let texture_infos: Vec<beyond_tap::texture_info::TextureInfo> = page.get_frames().values()
      .map(|frame| {
        packed_textures.push(frame.key.clone());
        beyond_tap::texture_info::TextureInfo {
          label: frame.key.clone(),
          x: frame.frame.x,
          y: frame.frame.y,
          w: frame.frame.w,
          h: frame.frame.h,
          is_rotated: frame.rotated,
        }
      }).collect();
    // Create the atlas info
    let atlas_info = beyond_tap::atlas_info::AtlasInfo {
      w: page.width(),
      h: page.height(),
      texture_infos: texture_infos,
    };
    // Serialize the atlas info
    let data = serde_json::to_string(&atlas_info)?;
    // Write to a file with the same name as the config file (outfile pfx) + page number
    let out_filename = format!("{}{}", outfile_pfx, i);
    let out_path_info = std::path::PathBuf::from(out_dir).join(out_filename.clone() + ".json");
    std::fs::write(out_path_info, data)?;
    // Export the atlas image
    let exporter = texture_packer::exporter::ImageExporter::export(page).map_err(|err| anyhow::anyhow!("{}", err))?;
    // Write to a file with the same name as the config file (outfile pfx) + page number
    let out_path_img = std::path::PathBuf::from(out_dir).join(out_filename + ".png");
    let mut file = std::fs::File::create(&out_path_img)?;
    exporter.write_to(&mut file, image::ImageFormat::Png)?;
    println!("Packed atlas of size {}x{} at {:?} containing: {:?}", atlas_info.w, atlas_info.h, out_path_img, packed_textures);
  }
  println!("Finished packing atlas(es)");
  Ok(())
}