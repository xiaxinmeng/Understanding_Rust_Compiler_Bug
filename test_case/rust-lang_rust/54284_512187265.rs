text
error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
 --> src/main.rs:2:49
  |
2 |     let _a = (0..255).flat_map(|x| (0..255).map(|y| (x, y))).collect::<Vec<_>>();
  |                                                 ^^^  - `x` is borrowed here
  |                                                 |
  |                                                 may outlive borrowed value `x`
  |
note: closure is returned here
 --> src/main.rs:2:36
  |
2 |     let _a = (0..255).flat_map(|x| (0..255).map(|y| (x, y))).collect::<Vec<_>>();
  |                                    ^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
  |
2 |     let _a = (0..255).flat_map(|x| (0..255).map(move |y| (x, y))).collect::<Vec<_>>();
  |                                                 ^^^^^^^^
