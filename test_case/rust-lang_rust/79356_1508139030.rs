
error: generic `Self` types are currently not permitted in anonymous constants
 --> src/lib.rs:8:11
  |
8 |     [(); <Self as Foo>::N]: ;
  |           ^^^^

error: could not compile `playground` (lib) due to previous error
