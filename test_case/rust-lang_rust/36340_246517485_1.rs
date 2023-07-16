
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
 --> src/lib.rs:3:24
  |
3 |         unsafe { split.get_unchecked(0) },
  |                        ^^^^^^^^^^^^^

error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
 --> src/lib.rs:4:24
  |
4 |         unsafe { split.get_unchecked(1) },
  |                        ^^^^^^^^^^^^^
  |
help: consider using an explicit lifetime parameter as shown: fn foo<'a>(split: &'a [&'a [u8]]) -> (&'a [u8], &'a [u8])
 --> src/lib.rs:1:1
  |
1 | fn foo<'a>(split: &[&'a [u8]]) -> (&'a [u8], &'a [u8]) {
  | ^

error: aborting due to 2 previous errors
