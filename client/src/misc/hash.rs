use std::hash::Hasher;

/// TypeIds are already hashes, so when using a TypeId as a key,
/// there is no point in touching the bits. `TypeIdHasher` holds
/// the inner u64 of the TypeId and returns it untouched.
#[derive(Default)]
pub struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
  fn write(&mut self, _: &[u8]) {
    unreachable!("TypeId uses `write_u64`");
  }
  #[inline]
  fn write_u64(&mut self, id: u64) {
    self.0 = id;
  }
  #[inline]
  fn finish(&self) -> u64 {
    self.0
  }
}