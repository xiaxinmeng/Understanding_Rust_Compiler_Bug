
error: implementation of `Helper` is not general enough
  --> src/main.rs:36:5
   |
36 |     par_iter_with_setup(v, setup, |iter| ())
   |     ^^^^^^^^^^^^^^^^^^^ implementation of `Helper` is not general enough
   |
   = note: `for<'a> fn(&'a Vec<String>) -> Vec<&'a str> {setup}` must implement `Helper<'0, Vec<String>>`, for any lifetime `'0`...
   = note: ...but it actually implements `Helper<'1, Vec<String>>`, for some specific lifetime `'1`
