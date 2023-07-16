
error: implementation of `X` is not general enough
  --> /home/nmatsakis/tmp/issue-57642.rs:31:13
   |
3  | / trait X {
4  | |     type G;
5  | |     fn make_g() -> Self::G;
6  | | }
   | |_- trait `X` defined here
...
31 |       let x = <fn (&())>::make_g();
   |               ^^^^^^^^^^^^^^^^^^ implementation of `X` is not general enough
   |
   = note: `X` would have to be implemented for the type `for<'r> fn(&'r ())`
   = note: ...but `X` is actually implemented for the type `fn(&'0 ())`, for some specific lifetime `'0`
