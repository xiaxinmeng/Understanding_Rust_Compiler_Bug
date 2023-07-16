
~ ❯❯❯ RUST_BACKTRACE=1 ./test                                            ✘ 101 
thread 'main' panicked at 'explicit panic', test.rs:2:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: test::main
   7: std::rt::lang_start::{{closure}}
   8: std::panicking::try::do_call
   9: __rust_maybe_catch_panic
  10: std::rt::lang_start_internal
  11: std::rt::lang_start
  12: main
