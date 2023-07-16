
thread '<unnamed>' panicked at 'malice is panicking', src/main.rs:14:5
stack backtrace:
   0: std::panicking::begin_panic
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:497
   1: test_crate::malice
             at ./src/main.rs:14
   2: test_crate::bob
             at ./src/main.rs:10
   3: test_crate::alice::{{closure}}
             at ./src/main.rs:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'malice is panicking', src/main.rs:14:5
stack backtrace:
   0: std::panicking::begin_panic
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:497
   1: test_crate::malice
             at ./src/main.rs:14
   2: test_crate::bob
             at ./src/main.rs:10
   3: test_crate::main
             at ./src/main.rs:21
   4: core::ops::function::FnOnce::call_once
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
