
error[E0282]: type annotations needed
--> src/lib.rs:6:31
  |
6 |     fn method(v: Vec<u8>) { v.len(); }
  |                               ^^^ cannot infer type
  |
  = note: type must be known at this point

error[E0599]: no method named `len` found for struct `Vec<u8>` in the current scope
--> src/lib.rs:6:31
  |
6 |     fn method(v: Vec<u8>) { v.len(); }
  |                               ^^^ private field, not a method
  |
  = note: `v` is a function, perhaps you wish to call it
