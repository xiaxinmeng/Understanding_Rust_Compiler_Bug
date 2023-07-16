
error[E0373]: closure may outlive the current function, but it borrows `a`, which is owned by the current function
 --> src/main.rs:5:6
  |
5 |     (|| { *a; }, a)
  |      ^^    - `a` is borrowed here
  |      |
  |      may outlive borrowed value `a`
  |
note: closure is returned here
 --> src/main.rs:5:5
  |
5 |     (|| { *a; }, a)
  |     ^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `a` (and any other referenced variables), use the `move` keyword
  |
5 |     (move || { *a; }, a)
  |      ^^^^^^^
