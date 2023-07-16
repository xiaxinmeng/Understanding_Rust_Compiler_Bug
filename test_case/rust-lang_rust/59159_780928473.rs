rust
let parent_name = self.paths.get(&parent).expect(&error);
if param {
  self.paths.insert(id, format!("{}/:{}", parent_name, name));
} else {
  self.paths.insert(id, format!("{}/{}", parent_name, name));
}
