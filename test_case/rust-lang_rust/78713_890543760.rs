
C:\Users\lukas>rustc +1.47.0 panic_unwinding.rs

C:\Users\lukas>set RUST_BACKTRACE=1

C:\Users\lukas>panic_unwinding.exe
thread '<unnamed>' panicked at 'malice is panicking!', panic_unwinding.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
   1: panic_unwinding::alice::{{closure}}
   2: panic_unwinding::alice::{{closure}}
   3: panic_unwinding::alice::{{closure}}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'malice is panicking!', panic_unwinding.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
   1: panic_unwinding::alice::{{closure}}
   2: panic_unwinding::alice::{{closure}}
   3: panic_unwinding::alice::{{closure}}
   4: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

C:\Users\lukas>rustc +1.54.0 panic_unwinding.rs

C:\Users\lukas>panic_unwinding.exe
thread '<unnamed>' panicked at 'malice is panicking!', panic_unwinding.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
   1: panic_unwinding::alice::{{closure}}
   2: panic_unwinding::alice::{{closure}}
   3: panic_unwinding::alice::{{closure}}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'malice is panicking!', panic_unwinding.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
   1: panic_unwinding::alice::{{closure}}
   2: panic_unwinding::alice::{{closure}}
   3: panic_unwinding::alice::{{closure}}
   4: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
