/// `include_bytes` but with respect to the cargo manifest directory.
#[macro_export]
macro_rules! include_wrt_manifest {
  ($path:expr) => {
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path))
  };
}