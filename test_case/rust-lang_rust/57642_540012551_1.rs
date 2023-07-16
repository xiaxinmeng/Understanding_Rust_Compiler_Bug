
error: implementation of `Y` is not general enough
  --> /home/nmatsakis/tmp/issue-57642.rs:36:13
   |
16 | / trait Y {
17 | |     type F;
18 | |     fn make_f() -> Self::F;
19 | | }
   | |_- trait `Y` defined here
...
36 |       let x = <fn (&())>::make_f();
   |               ^^^^^^^^^^^^^^^^^^ implementation of `Y` is not general enough
   |
   = note: `Y` would have to be implemented for the type `for<'r> fn(&'r ())`
   = note: ...but `Y` is actually implemented for the type `fn(&'0 ())`, for some specific lifetime `'0`
