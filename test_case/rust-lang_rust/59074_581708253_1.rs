
error[E0373]: closure may outlive the current function, but it borrows `xs`, which is owned by the current function
 --> src/lib.rs:5:5
  |
5 |     || xs
  |     ^^ -- `xs` is borrowed here
  |     |
  |     may outlive borrowed value `xs`
  |
note: closure is returned here
 --> src/lib.rs:5:5
  |
5 |     || xs
  |     ^^^^^
help: to force the closure to take ownership of `xs` (and any other referenced variables), use the `move` keyword
  |
5 |     move || xs
  |     ^^^^^^^
