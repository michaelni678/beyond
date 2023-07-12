const TAP_CONFIG_DIR: &str = "configs/tap";

fn main() {
  // Collect the config file paths in the directory
  let config_file_paths: Vec<std::path::PathBuf> = std::fs::read_dir(TAP_CONFIG_DIR).expect("failed to read dir")
    .map(|dir_entry| {
      dir_entry.ok().map(|entry| entry.path())
    })
    .flatten()
    .filter(|path| {
      path.extension().map_or_else(|| false, |ext| ext == "json")
    })
    .collect();
  println!("Detected {} config file(s) in {}: {:?}", config_file_paths.len(), TAP_CONFIG_DIR, config_file_paths);
  // Loop through the config files
  for config_file_path in config_file_paths.into_iter() {
    if let Err(err) = beyond_tap::run(config_file_path.clone()) {
      eprintln!("Skipping config file {:?} because: {}", config_file_path, err);
    } else {
      println!("Finished using config file {:?}", config_file_path);
    }
  }
}