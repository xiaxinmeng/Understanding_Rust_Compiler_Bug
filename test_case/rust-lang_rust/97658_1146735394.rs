plain
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.126
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
error: cannot find a built-in macro with name `format_args_ln`
    |
902 | /     macro_rules! format_args_ln {
902 | /     macro_rules! format_args_ln {
903 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
904 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____^

   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
