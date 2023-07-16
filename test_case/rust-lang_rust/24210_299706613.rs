
error[E0507]: cannot move out of captured outer variable in an `Fn` closure
  --> test.rs:11:37
   |
11 |     let x = Moves;   direct(|| drop(x));
   |         - captured outer variable   ^ cannot move out of captured outer variable in an `Fn` closure

error: aborting due to previous error
