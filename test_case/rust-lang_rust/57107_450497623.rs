
error[E0594]: cannot assign to immutable thread-local static item
  --> t.rs:11:5
   |
11 |     S = "after"; //~ ERROR cannot assign to immutable thread-local static item
   |     ^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
