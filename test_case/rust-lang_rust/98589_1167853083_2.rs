
error[E0309]: the parameter type `T` may not live long enough
  --> src/lib.rs:14:13
   |
14 |     let f = |x: T| -> Box<dyn Display + 'a> { lives_as_long::<'a, T>(x) };
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound...
   |
10 | fn test<'a, T: Display + 'a>(x: T)
   |                        ++++
