
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> test.rs:13:30
   |
13 |     let y = Moves;   blanket(|| drop(y));
   |                              ^^^^^^^^^^
   |
note: the requirement to implement `Fn` derives from here
  --> test.rs:13:22
   |
13 |     let y = Moves;   blanket(|| drop(y));
   |                      ^^^^^^^

error: aborting due to previous error
