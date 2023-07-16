
error: `<` is interpreted as a start of generic arguments for `u16`, not a comparison
 --> test.rs:6:62
  |
6 |         println!("{}",(input.chars().nth(i).unwrap()) as u16 << 8);
  |                                                              ^^ - interpreted as generic arguments
  |                                                              |
  |                                                              not interpreted as comparison
  |
help: if you want to compare the casted value then write:
  |
6 |         println!("{}",((input.chars().nth(i).unwrap()) as u16) << 8);
  |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
