
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
 --> src/main.rs:3:9
  |
3 | let f = || x += 1;
  |         ^^ - closure is `FnMut` because it mutates the variable `x` here
  |         |
  |         this closure implements `FnMut`, not `Fn`
4 | let y = &f as  &dyn Fn();
  |         -- the requirement to implement `Fn` derives from here

For more information about this error, try `rustc --explain E0525`. 
