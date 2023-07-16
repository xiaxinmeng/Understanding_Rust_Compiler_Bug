
thread 'main' panicked at 'Test', /home/mark/Build/rust/library/std/src/panicking.rs:457:65
stack backtrace:
   0: std::panicking::begin_panic
             at ./Build/rust/library/std/src/panicking.rs:456
   1: t::bar
             at ./t.rs:6
   2: t::main
             at ./t.rs:2
   3: core::ops::function::FnOnce::call_once
             at ./Build/rust/library/core/src/ops/function.rs:233
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
