
error[E0432]: unresolved import `inner`
  --> bar.rs:12:13
   |
12 |     pub use inner::*;
   |             ^^^^^ help: a similar path exists: `self::inner`

error[E0432]: unresolved import `crate::oops::OhNo`
 --> bar.rs:4:13
  |
4 |     pub use crate::oops::OhNo;
  |             ^^^^^^^^^^^^^^^^^ no `OhNo` in `oops`
  |
help: consider importing this enum instead
  |
4 |     pub use oops::inner::OhNo;
  |             ~~~~~~~~~~~~~~~~~

error: Compilation failed, aborting rustdoc

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.
