
error: any use of this value will cause an error
  --> src/lib.rs:22:1
   |
14 | /         const _: () = {
15 | |             //if !$cond {
16 | |                 $crate::core_::panic!($($t)+)
17 | |             //}
18 | |         };
   | |__________-
...
22 |   static_assert!(1 + 1 == 3);
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |   |
   |   the evaluated program panicked at 'assertion failed: 1 + 1 == 3', src/lib.rs:22:1
   |   in this macro invocation
   |
note: lint level defined here
  --> src/lib.rs:13:18
   |
13 |         #[forbid(const_err)]
   |                  ^^^^^^^^^
...
22 | static_assert!(1 + 1 == 3);
   | --------------------------- in this macro invocation
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
