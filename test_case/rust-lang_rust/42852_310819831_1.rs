
thread 'main' panicked at 'assertion failed: `(left == right)` (left: `4485943328`, right: `0`)', src/libstd/sync/mpsc/shared.rs:253
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic
   5: std::panicking::begin_panic_fmt
   6: <std::sync::mpsc::shared::Packet<T>>::decrement
   7: <std::sync::mpsc::shared::Packet<T>>::recv
   8: <std::sync::mpsc::Receiver<T>>::recv_max_until
   9: <std::sync::mpsc::Receiver<T>>::recv_timeout
  10: channel_bug::main
  11: __rust_maybe_catch_panic
  12: std::rt::lang_start
  13: main
