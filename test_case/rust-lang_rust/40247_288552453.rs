rust
  fn walk_paths(&self, parent: &FolderPath) -> impl Iterator<Item=&FolderPath> {
    self.nodes.keys().filter(|p| p.is_child_of(parent))
  }
