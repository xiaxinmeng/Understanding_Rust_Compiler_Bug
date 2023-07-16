plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `panic` is not yet stable as a const fn
    |
23  | / pub macro panic_2015 {
24  | |     () => (
25  | |         $crate::panicking::panic("explicit panic")
---
12  | |     };
13  | | }
    | |_- in this expansion of `panic!` (#2)
...
214 | / macro_rules! debug_assert {
215 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })
    | |                                                           |
    | |                                                           in this macro invocation (#2)
    | |                                                           in this macro invocation (#3)
216 | | }
216 | | }
    | |_- in this expansion of `debug_assert!` (#1)
    |
   ::: library/core/src/hint.rs:49:5
    |
49  |       debug_assert!(false, "entered unreachable code");
    |
    = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to previous error
