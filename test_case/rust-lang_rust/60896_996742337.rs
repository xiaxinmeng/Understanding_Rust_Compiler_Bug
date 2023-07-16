rust
struct Person {
  /// Unique integer id, Eq, Hash and Ord are defined in terms of ids 
  id: u32, 
  email: String,
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool { self.id == other.id }
}
