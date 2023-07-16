plain
    Checking test v0.0.0 (/checkout/library/test)
    Checking rand_core v0.5.1
    Checking rand_chacha v0.2.2
    Checking rand_xorshift v0.2.0
error[E0560]: struct `ConsoleTestState` has no field named `log_out`
    |
799 |         log_out: None,
    |         ^^^^^^^ `ConsoleTestState` does not have this field
    |
    |
    = note: available fields are: `total`, `passed`, `failed`, `ignored`, `filtered_out` ... and 7 others
For more information about this error, try `rustc --explain E0560`.
error: could not compile `test` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:42
