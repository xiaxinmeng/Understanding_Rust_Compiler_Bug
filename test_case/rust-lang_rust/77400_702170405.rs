rust
enum Profile {
  Compiler,
  Codegen,
  Library,
  User,
}

// keeps case-insensitive matching, etc.
impl FromStr for Profile { ... }
impl Display for Profile { ... }
impl Profile {
  fn include_path(&self, src_path: &Path) -> PathBuf
}
