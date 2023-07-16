
error[E0282]: type annotations needed
 --> src/main.rs:3:9
  |
3 |     lst.sort_by_key(|&(v, _)| v.iter().sum());
  |                               ^^^^^^^^^^^^^^ cannot infer type for `B`
  = note: could not infer type `B` for method `fn sort_by_key<B, F>(&mut self, _: F) where B: Ord, F: FnMut(&T) -> B`
