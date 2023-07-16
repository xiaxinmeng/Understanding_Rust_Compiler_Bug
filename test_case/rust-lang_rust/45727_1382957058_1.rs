
error[E0631]: type mismatch in closure arguments
 --> src/main.rs:2:24
  |
2 |     let _ = (-10..=10).find(|x: i32| x.signum() == 0);
  |                        ^^^^ -------- found signature defined here
  |                        |
  |                        expected due to this
  |
  = note: expected closure signature `for<'a> fn(&'a {integer}) -> _`
             found closure signature `fn(i32) -> _`
note: required by a bound in `find`
 --> /rustc/0b90256ada21c6a81b4c18f2c7a23151ab5fc232/library/core/src/iter/traits/iterator.rs:2712:5
