
error[E0433]: failed to resolve. Maybe a missing `extern crate noop_helper;`?
  --> src/lib.rs:6:17
   |
6  |         $crate::noop_helper!{}
   |                 ^^^^^^^^^^^ Maybe a missing `extern crate noop_helper;`?
...
17 |     noop!{}
   |     ------- in this macro invocation
