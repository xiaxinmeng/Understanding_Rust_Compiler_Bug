
   Compiling playground v0.0.1 (/playground)
error[E0623]: lifetime mismatch
 --> src/lib.rs:8:49
  |
8 |     fn use_pattern(&self, pattern: &Pattern) -> impl Iterator<Item = Foo<'_>> {
  |                                    --------     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |                                    |            |
  |                                    |            ...but data from `pattern` is returned here
  |                                    this parameter and the return type are declared with different lifetimes...

error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
