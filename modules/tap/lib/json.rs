use std::io::Read;

// Read a JSON and convert it into a serde_json::Value
pub fn read_json(file_path: std::path::PathBuf) -> anyhow::Result<serde_json::Value> {
  // Read the data
  let mut file = std::fs::File::open(file_path)?;
  let mut data = String::new();
  file.read_to_string(&mut data)?;
  Ok(serde_json::from_str(&data)?)
}

// A macro to get the field of a serde_json::Value
// Usage: get_field!(<config>, <field_name>, <type>)
macro_rules! get_field {
  ($config:expr, $name:expr, $method:ident) => {
    $config.get($name).ok_or_else(|| anyhow::anyhow!(format!("failed to get {} from the config", $name)))?
      .$method().ok_or_else(|| anyhow::anyhow!(format!("failed to convert {}", $name)))
  };
}
pub(crate) use get_field;