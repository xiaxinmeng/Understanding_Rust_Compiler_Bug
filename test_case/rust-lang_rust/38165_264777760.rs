
thread 'main' panicked at 'explicit panic', t.rs:2
stack backtrace:
   1:    std::sys::imp::backtrace::tracing::imp::write
   2:    std::panicking::default_hook::{{closure}}
   3:    std::panicking::default_hook
   4:    std::panicking::rust_panic_with_hook
   5:    std::panicking::begin_panic
             at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:417
   6:    t::main
             at /tmp/pote/t.rs:2
   7:    __rust_maybe_catch_panic
   8:    std::rt::lang_start
   9:    main
  10:    __libc_start_main
  11:    _start
