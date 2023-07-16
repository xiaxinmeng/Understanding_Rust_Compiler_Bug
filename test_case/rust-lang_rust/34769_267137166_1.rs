
% ./34769.sh
   Compiling a v0.0.1 (file:///home/cody/rs-34769/a)
   Compiling b v0.0.1 (file:///home/cody/rs-34769/b)
   Compiling c v0.0.1 (file:///home/cody/rs-34769/c)
error[E0308]: mismatched types
 --> src/lib.rs:7:21
  |
7 |    takes_foo_from_a(x)
  |                     ^ expected struct `b::Foo`, found a different struct `b::Foo`
  |
  = note: expected type `b::Foo` (struct `b::Foo`)
  = note:    found type `b::Foo` (struct `b::Foo`)
note: Perhaps two different versions of crate `a` are being used?
 --> src/lib.rs:7:21
  |
7 |    takes_foo_from_a(x)
  |                     ^

error: aborting due to previous error

error: Could not compile `c`.

To learn more, run the command again with --verbose.
