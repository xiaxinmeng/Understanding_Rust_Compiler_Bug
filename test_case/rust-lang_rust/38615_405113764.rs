
error[E0373]: closure may outlive the current function, but it borrows `log`, which is owned by the current function
 --> src/main.rs:5:19
  |
5 |     let handler = |msg: &String| -> () {
  |                   ^^^^^^^^^^^^^^^^^^^^ may outlive borrowed value `log`
6 |         log.push(format!("The messages was: {}", msg));
  |         --- `log` is borrowed here
help: to force the closure to take ownership of `log` (and any other referenced variables), use the `move` keyword
  |
5 |     let handler = move |msg: &String| -> () {
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
