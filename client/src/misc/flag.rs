/// A flag used to check for state change.
pub struct Flag<T> {
  value: T,
  is_dirty: bool,
}

impl<T> Flag<T> {
  /// Create a new flag. Initialized as dirty.
  pub fn new_dirty(value: T) -> Self {
    Self {
      value: value,
      is_dirty: true,
    }
  }
  /// Create a new flag. Initialized as clean.
  pub fn new_clean(value: T) -> Self {
    Self {
      value: value,
      is_dirty: false,
    }
  }
  /// Get whether the flag is dirty or not.
  #[inline]
  pub fn is_dirty(&self) -> bool {
    self.is_dirty
  }
  /// Get the value of the flag.
  #[inline]
  pub fn get(&self) -> &T {
    &self.value
  }
  /// Change the state of the flag clean.
  #[inline]
  pub fn clean(&mut self) {
    self.is_dirty = false;
  }
  /// Set the value of the flag.
  /// This automatically changes the flag to dirty.
  #[inline]
  pub fn set(&mut self, new_value: T) {
    self.value = new_value;
    self.is_dirty = true;
  }
}
