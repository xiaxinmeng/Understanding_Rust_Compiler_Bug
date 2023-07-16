
error[E0373]: closure may outlive the current function, but it borrows `i`, which is owned by the current function
 --> src/main.rs:2:16
  |
2 |     |i: u32| { || i };
  |                ^^ - `i` is borrowed here
  |                |
  |                may outlive borrowed value `i`
  |
note: closure is returned here
 --> src/main.rs:2:16
  |
2 |     |i: u32| { || i };
  |                ^^^^
help: to force the closure to take ownership of `i` (and any other referenced variables), use the `move` keyword
  |
2 |     |i: u32| { move || i };
  |                ^^^^^^^
