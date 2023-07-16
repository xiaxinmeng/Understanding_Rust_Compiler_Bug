
error[E0308]: method not compatible with trait
 --> src/lib.rs:6:5
  |
6 |     fn f(_: &'static ()) {}
  |     ^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
  |
  = note: expected fn pointer `fn(&'a ())`
             found fn pointer `fn(&'static ())`
note: the lifetime `'a` as defined here...
 --> src/lib.rs:6:5
  |
6 |     fn f(_: &'static ()) {}
  |     ^^^^^^^^^^^^^^^^^^^^
  = note: ...does not necessarily outlive the static lifetime
