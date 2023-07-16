
error[E0308]: mismatched types
 --> src/lib.rs:3:5
  |
2 | fn clone_map<T>(map1: &HashMap<String, T>) -> HashMap<String, T> {
  |                                               ------------------ expected `HashMap<String, T>` because of return type
3 |     map1.clone()
  |     ^^^^^^^^^^^^ expected struct `HashMap`, found reference
  |
  = note: expected struct `HashMap<String, T>`
          found reference `&HashMap<String, T>`
note: `HashMap<String, T>` does not implement `Clone`, so `&HashMap<String, T>` was cloned instead
 --> src/lib.rs:3:5
  |
3 |     map1.clone()
  |     ^^^^
