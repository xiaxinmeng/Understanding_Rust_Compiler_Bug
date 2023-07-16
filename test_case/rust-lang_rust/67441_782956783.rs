
    --> library/core/src/macros/mod.rs:18:38
     |
7    | / macro_rules! panic {
8    | |     () => (
9    | |         $crate::panic!("explicit panic")
10   | |     );
...    |
18   | |         $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))
     | |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation (#2)
19   | |     );
20   | | }
     | |_- in this expansion of `panic!` (#1)
...
761  | /     macro_rules! format_args {
762  | |         ($fmt:expr) => {{ /* compiler built-in */ }};
763  | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
764  | |     }
     | |_____- in this expansion of `$crate::format_args!` (#2)
     | 
    ::: library/core/src/option.rs:1294:5
     |
1294 |       panic!("{}", msg)
     |       ----------------- in this macro invocation (#1)
