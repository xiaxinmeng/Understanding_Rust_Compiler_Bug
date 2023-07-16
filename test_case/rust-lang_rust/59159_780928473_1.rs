rust
let parent_name = self.paths.get(&parent).expect(&error);
if param {
  let id_name = format!("{}/:{}", parent_name, name);
  self.paths.insert(id, id_name);
} else {
  let id_name = format!("{}/{}", parent_name, name);
  self.paths.insert(id, id_name);
}
