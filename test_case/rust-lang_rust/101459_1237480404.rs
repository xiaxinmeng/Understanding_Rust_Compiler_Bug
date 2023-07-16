

error[E0507]: cannot move out of a captured variable in an `Fn` closure
  --> src/main.rs:23:18
   |
16 |     let b = a.clone();
   |         - this variable is being captured
...
22 |     badfunc(move || {
   |             ------- the variable `b` is captured by this `Fn` closure
23 |         printlen(b);
   |                  ^ move occurs because `b` has type `String`, which does not implement the `Copy` trait
   |
note: you can only move out of captured variables in `FnOnce` closures, but never in `FnMut` closures or `Fn` closures

For more information about this error, try `rustc --explain E0507`.
