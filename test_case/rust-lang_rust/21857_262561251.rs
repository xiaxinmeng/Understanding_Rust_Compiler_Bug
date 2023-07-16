rust
error[E0281]: type mismatch: the type `[closure@<anon>:3:23: 3:39]` implements the trait `for<'r> std::ops::FnMut<(&'r {integer},)>`, but the trait `for<'r, 'r> std::ops::FnMut<(&'r {integer}, &'r {integer})>` is required (expected a tuple with 2 elements, found one with 1 elements)
 --> <anon>:3:15
  |
3 |     [1, 2, 3].sort_by(|tuple| panic!());
  |               ^^^^^^^

error[E0281]: type mismatch: the type `[closure@<anon>:3:23: 3:39]` implements the trait `for<'r> std::ops::FnOnce<(&'r {integer},)>`, but the trait `for<'r, 'r> std::ops::FnOnce<(&'r {integer}, &'r {integer})>` is required (expected a tuple with 2 elements, found one with 1 elements)
 --> <anon>:3:15
  |
3 |     [1, 2, 3].sort_by(|tuple| panic!());
  |               ^^^^^^^

error[E0277]: the trait bound `for<'r, 'r> {integer}: std::ops::FnMut<(&'r {integer}, &'r {integer})>` is not satisfied
 --> <anon>:4:15
  |
4 |     [1, 2, 3].sort_by(1);
  |               ^^^^^^^ the trait `for<'r, 'r> std::ops::FnMut<(&'r {integer}, &'r {integer})>` is not implemented for `{integer}`
  |
  = help: the following implementations were found:
  = help:   <&'a F as std::ops::FnMut<A>>
  = help:   <&'a mut F as std::ops::FnMut<A>>
  = help:   <core::str::LinesAnyMap as std::ops::FnMut<(&'a str,)>>

error[E0277]: the trait bound `for<'r, 'r> {integer}: std::ops::FnOnce<(&'r {integer}, &'r {integer})>` is not satisfied
 --> <anon>:4:15
  |
4 |     [1, 2, 3].sort_by(1);
  |               ^^^^^^^ the trait `for<'r, 'r> std::ops::FnOnce<(&'r {integer}, &'r {integer})>` is not implemented for `{integer}`
  |
  = help: the following implementations were found:
  = help:   <&'a F as std::ops::FnOnce<A>>
  = help:   <&'a mut F as std::ops::FnOnce<A>>
  = help:   <core::str::LinesAnyMap as std::ops::FnOnce<(&'a str,)>>
  = help:   <Box<std::boxed::FnBox<A, Output=R> + 'a> as std::ops::FnOnce<A>>
  = help: and 2 others
