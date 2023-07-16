
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> src/main.rs:4:18
  |
4 |     v.iter().map(|e| if v.len() > 10 { e * 2 } else { e * 4 })
  |                  ^^^    - `v` is borrowed here
  |                  |
  |                  may outlive borrowed value `v`
  |
note: closure is returned here
 --> src/main.rs:4:5
  |
4 |     v.iter().map(|e| if v.len() > 10 { e * 2 } else { e * 4 })
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
4 |     v.iter().map(move |e| if v.len() > 10 { e * 2 } else { e * 4 })
  |                  ++++
