
error[E0508]: cannot move out of type `[std::boxed::Box<i32>; 1]`, a non-copy array
 --> src/main.rs:2:13
  |
2 |     let x = [Box::new(0)][0];
  |             ^^^^^^^^^^^^^^^^
  |             |
  |             help: consider using a reference instead `&[Box::new(0)][0]`
  |             cannot move out of here
