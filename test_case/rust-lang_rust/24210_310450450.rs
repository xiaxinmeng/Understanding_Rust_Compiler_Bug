
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> test.rs:12:28
   |
12 |     let y = Moves; blanket(|| drop(y));
   |                    ------- ^^^^^^^^^^
   |                    |
   |                    the requirement to implement `Fn` derives from here
   |
note: closure is `FnOnce` because it moves the variable `y` out of its environment
  --> test.rs:12:36
   |
12 |     let y = Moves; blanket(|| drop(y));
   |                                    ^

error: aborting due to previous error(s)
