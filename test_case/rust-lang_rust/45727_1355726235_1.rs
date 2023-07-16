
error[E0631]: type mismatch in closure arguments
 --> src/lib.rs:2:24
  |
2 |     let _ = (-10..=10).find(|x: i32| x.signum() == 0);
  |                        ^^^^ -------- found signature defined here
  |                        |
  |                        expected due to this
  |
  = note: expected closure signature `for<'a> fn(&'a {integer}) -> _`
             found closure signature `fn(i32) -> _`
note: required by a bound in `find`

For more information about this error, try `rustc --explain E0631`.
