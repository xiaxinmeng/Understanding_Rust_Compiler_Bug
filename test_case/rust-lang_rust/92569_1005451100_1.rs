rs
error[E0609]: no field `0` on type `fn(u8) -> Foo {Foo}`
  --> test.rs:12:15
   |
12 |     thing.bar.0;
   |     --------- ^
   |     |
   |     help: call the constructor: `(thing.bar)(_)`
   |
   = help: placeholder

error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
