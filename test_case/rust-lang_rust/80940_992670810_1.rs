
error: cannot find macro `private` in this scope
  --> src/lib.rs:9:13
   |
9  |             private!();
   |             ^^^^^^^
...
14 | crate::public!();
   | ---------------- in this macro invocation
   |
   = help: have you added the `#[macro_use]` on the module/import?
   = note: this error originates in the macro `crate::public` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused macro definition: `private`
 --> src/lib.rs:2:18
  |
2 |     macro_rules! private {
  |                  ^^^^^^^
  |
  = note: `#[warn(unused_macros)]` on by default
