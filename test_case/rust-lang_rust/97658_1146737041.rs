plain
916  | |         };
...    |
919  | |         };
920  | |     }
     | |_____- in this expansion of `$crate::format_args_ln!` (#2)
927  | /     macro_rules! format_args_nl {
927  | /     macro_rules! format_args_nl {
928  | |         ($fmt:expr) => {{ /* compiler built-in */ }};
     | |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
929  | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `format_args_nl!` (#3)
     |
    ::: library/std/src/process.rs:2164:9
     |
     |
2164 |           eprintln!("Error: {err:?}");
     |
    ::: library/std/src/macros.rs:167:1
     |
167  | / macro_rules! eprintln {
167  | / macro_rules! eprintln {
168  | |     () => {
169  | |         $crate::eprint!("\n")
171  | |     ($($arg:tt)*) => {{
171  | |     ($($arg:tt)*) => {{
172  | |         $crate::io::_eprint($crate::format_args_ln!($($arg)*));
173  | |     }};
174  | | }
174  | | }
     | |_- in this expansion of `eprintln!` (#1)
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:928:25
    |
552 | / macro_rules! writeln {
552 | / macro_rules! writeln {
553 | |     ($dst:expr $(,)?) => {
554 | |         $crate::write!($dst, "\n")
555 | |     };
556 | |     ($dst:expr, $($arg:tt)*) => {{
557 | |         let result = $dst.write_fmt($crate::format_args_ln!($($arg)*));
558 | |         result
559 | |     }};
560 | | }
    | |_- in this expansion of `writeln!` (#1)
---
916 | |         };
...   |
919 | |         };
920 | |     }
    | |_____- in this expansion of `$crate::format_args_ln!` (#2)
927 | /     macro_rules! format_args_nl {
927 | /     macro_rules! format_args_nl {
928 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
    | |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
929 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `format_args_nl!` (#3)
    |
   ::: library/std/src/sys_common/backtrace.rs:59:5
    |
    |
59  |       writeln!(fmt, "stack backtrace:")?;

error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:928:25
    |
    |
552 | / macro_rules! writeln {
553 | |     ($dst:expr $(,)?) => {
554 | |         $crate::write!($dst, "\n")
555 | |     };
556 | |     ($dst:expr, $($arg:tt)*) => {{
557 | |         let result = $dst.write_fmt($crate::format_args_ln!($($arg)*));
558 | |         result
559 | |     }};
560 | | }
    | |_- in this expansion of `writeln!` (#1)
---
916 | |         };
...   |
919 | |         };
920 | |     }
    | |_____- in this expansion of `$crate::format_args_ln!` (#2)
927 | /     macro_rules! format_args_nl {
927 | /     macro_rules! format_args_nl {
928 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
    | |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
929 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `format_args_nl!` (#3)
    |
   ::: library/std/src/sys_common/backtrace.rs:105:9
    |
---

error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:928:25
    |
552 | / macro_rules! writeln {
553 | |     ($dst:expr $(,)?) => {
554 | |         $crate::write!($dst, "\n")
555 | |     };
556 | |     ($dst:expr, $($arg:tt)*) => {{
557 | |         let result = $dst.write_fmt($crate::format_args_ln!($($arg)*));
558 | |         result
559 | |     }};
560 | | }
    | |_- in this expansion of `writeln!` (#1)
---
916 | |         };
...   |
919 | |         };
920 | |     }
    | |_____- in this expansion of `$crate::format_args_ln!` (#2)
927 | /     macro_rules! format_args_nl {
927 | /     macro_rules! format_args_nl {
928 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
    | |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
929 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `format_args_nl!` (#3)
    |
   ::: library/std/src/panicking.rs:286:17
    |
    |
286 |           let _ = writeln!(err, "thread '{name}' panicked at '{msg}', {location}");

error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:928:25
    |
    |
552 | / macro_rules! writeln {
553 | |     ($dst:expr $(,)?) => {
554 | |         $crate::write!($dst, "\n")
555 | |     };
556 | |     ($dst:expr, $($arg:tt)*) => {{
557 | |         let result = $dst.write_fmt($crate::format_args_ln!($($arg)*));
558 | |         result
559 | |     }};
560 | | }
    | |_- in this expansion of `writeln!` (#1)
---
916 | |         };
...   |
919 | |         };
920 | |     }
    | |_____- in this expansion of `$crate::format_args_ln!` (#2)
927 | /     macro_rules! format_args_nl {
927 | /     macro_rules! format_args_nl {
928 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
    | |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
929 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `format_args_nl!` (#3)
    |
   ::: library/std/src/panicking.rs:299:29
    |
