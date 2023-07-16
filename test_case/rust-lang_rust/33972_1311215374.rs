
error[E0053]: method `index` has an incompatible type for trait
 --> src/lib.rs:6:43
  |
6 |     fn index<'a>(&'a self, index: u16) -> u8 {
  |                                           ^^
  |                                           |
  |                                           expected `&u8`, found `u8`
  |                                           help: change the output type to match the trait: `&u8`
  |
  = note: expected fn pointer `fn(&S, _) -> &u8`
             found fn pointer `fn(&S, _) -> u8`
