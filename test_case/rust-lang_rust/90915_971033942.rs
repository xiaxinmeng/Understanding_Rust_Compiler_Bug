
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
