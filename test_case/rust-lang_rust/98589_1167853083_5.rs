
error[E0309]: the parameter type `T` may not live long enough
  --> src/lib.rs:15:13
   |
15 |       let f = |x: T| {
   |  _____________^
16 | |         r = Some(lives_as_long::<'a, T>(x));
17 | |     };
   | |_____^ ...so that the type `T` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound...
   |
10 | fn test<'a, T: Display + 'a>(x: T)
   |                        ++++
