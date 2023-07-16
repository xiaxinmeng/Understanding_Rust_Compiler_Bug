rust
let old_value = self.values.insert(key, value);
assert!(old_value.is_none());
