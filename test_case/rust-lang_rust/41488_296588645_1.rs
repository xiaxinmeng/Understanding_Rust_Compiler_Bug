
error[E0308]: mismatched types
 --> ...\test.rs:3:24
  |
3 |     [1, 2, 3].sort_by(|(x, y)| panic!());
  |                        ^^^^^^ expected &{integer}, found tuple
  |
  = note: expected type `&{integer}`
             found type `(_, _)`

error[E0281]: type mismatch: the type `[closure@...\test.rs:3:23: 3:40]` implements the trait `for<'r> std::ops::FnMut<(&'r {integer},)>`, but the trait `for<'r, 'r> std::ops::FnMut<(&'r {integer}, &'r {integer})>` is required (expected a tuple with 2 elements, found one with 1 elements)
 --> ...\test.rs:3:15
  |
3 |     [1, 2, 3].sort_by(|(x, y)| panic!());
  |               ^^^^^^^

error[E0281]: type mismatch: the type `[closure@...\test.rs:3:23: 3:40]` implements the trait `for<'r> std::ops::FnOnce<(&'r {integer},)>`, but the trait `for<'r, 'r> std::ops::FnOnce<(&'r {integer}, &'r {integer})>` is required (expected a tuple with 2 elements, found one with 1 elements)
 --> ...\test.rs:3:15
  |
3 |     [1, 2, 3].sort_by(|(x, y)| panic!());
  |               ^^^^^^^

error: aborting due to 3 previous errors
