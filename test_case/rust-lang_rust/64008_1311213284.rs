
error[E0373]: closure may outlive the current function, but it borrows `bar`, which is owned by the current function
 --> src/lib.rs:6:30
  |
6 |     Some(()).iter().flat_map(|()| {
  |                              ^^^^ may outlive borrowed value `bar`
7 |         Some(()).iter().map(|()| { bar; })
  |                                    --- `bar` is borrowed here
  |
note: closure is returned here
 --> src/lib.rs:6:5
  |
6 | /     Some(()).iter().flat_map(|()| {
7 | |         Some(()).iter().map(|()| { bar; })
8 | |     })
  | |______^
help: to force the closure to take ownership of `bar` (and any other referenced variables), use the `move` keyword
  |
6 |     Some(()).iter().flat_map(move |()| {
  |                              ++++
