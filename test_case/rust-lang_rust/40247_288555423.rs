rust
  pub fn walk_paths<'a>(&'a self, parent: FolderPath) -> impl Iterator<Item=&FolderPath> + 'a {
    self.nodes.keys().filter(move |p| p.is_child_of(&parent))
  }
