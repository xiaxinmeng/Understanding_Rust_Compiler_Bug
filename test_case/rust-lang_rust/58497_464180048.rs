
error[E0373]: closure may outlive the current function, but it borrows `bar`, which is owned by the current function
 --> src/lib.rs:5:5
  |
5 |     || bar
  |     ^^ --- `bar` is borrowed here
  |     |
  |     may outlive borrowed value `bar`
  |
note: closure is returned here
 --> src/lib.rs:5:5
  |
5 |     || bar
  |     ^^^^^^
help: to force the closure to take ownership of `bar` (and any other referenced variables), use the `move` keyword
  |
5 |     move || bar
  |     ^^^^^^^

error: aborting due to previous error
