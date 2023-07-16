
error: arbitrary expressions aren't allowed in patterns
  --> src/main.rs:11:48
   |
11 |         vec!(@force_expr_internal_do_not_use@: <[_]>::into_vec(box [$($x),+]))
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
18 |         vec![43] => {},
   |         -------- in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
