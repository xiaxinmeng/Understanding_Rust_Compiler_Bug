
> rustc .\src\main.rs
> .\main.exe
thread '<unnamed>' panicked at 'malice is panicking!', .\src\main.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
   1: main::alice::{{closure}}
   2: main::alice::{{closure}}
   3: main::alice::{{closure}}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'malice is panicking!', .\src\main.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
   1: main::alice::{{closure}}
   2: main::alice::{{closure}}
   3: main::alice::{{closure}}
   4: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
