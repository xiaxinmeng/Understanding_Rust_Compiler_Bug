plain
   Compiling libc v0.2.88
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0541]: unknown meta item 'reason'
    |
    |
614 |     #[stable(feature = "option_insert", reason = "newly added", since = "1.53.0")]
    |                                         ^^^^^^^^^^^^^^^^^^^^^^ expected one of `since`, `note`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0541`.
error: could not compile `core`
