
error: the `min` method cannot be invoked on a trait object
 --> src/main.rs:2:9
  |
2 |      *t.min().unwrap()
  |         ^^^
  |
  = note: you need `&mut dyn std::iter::Iterator<Item = &u64>` instead of `&dyn std::iter::Iterator<Item = &u64>`
