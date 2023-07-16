
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
 --> ../rust-prog/issue_26046.rs:4:19
  |
4 |       let closure = move || {
  |  ___________________^
5 | |         vec
6 | |     };
  | |_____^
  |
note: closure is `FnOnce` because it moves the variable `vec` out of its environment
 --> ../rust-prog/issue_26046.rs:5:9
  |
5 |         vec
  |         ^^^

error: aborting due to previous error(s)
