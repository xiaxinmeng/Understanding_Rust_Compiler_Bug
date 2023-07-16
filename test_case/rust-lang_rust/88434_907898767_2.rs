
warning: function is never used: `f`
 --> src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:8:10
  |
8 | const fn f<F>(_: &[u8], _: F) -> &[u8]
  |          ^
  |
  = note: `#[warn(dead_code)]` on by default

error[E0080]: evaluation of constant value failed
  --> src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:12:5
   |
6  | const _CONST: &[u8] = &f(&[], |_| {});
   |                        -------------- inside `_CONST` at src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:6:24
...
12 |     panic!()
   |     ^^^^^^^^
   |     |
   |     the evaluated program panicked at 'explicit panic', src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:12:5
   |     inside `f::<[closure@src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:6:31: 6:37]>` at /home/noble/Desktop/rust/library/std/src/panic.rs:18:9
   |
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
