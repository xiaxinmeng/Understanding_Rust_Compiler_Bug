
error[E0576]: cannot find method or associated constant `trim` in `str`
 --> src/lib.rs:6:24
  |
6 |         <Self as str>::trim(self);
  |                        ^^^^ `str` is not a trait

For more information about this error, try `rustc --explain E0576`.
