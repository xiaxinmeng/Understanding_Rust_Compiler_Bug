
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ()', src/libcore/result.rs:837
stack backtrace:
   1:    std::sys::imp::backtrace::tracing::imp::write::hfc2edb670e5eda97
   2:    std::panicking::default_hook::{{closure}}::hc66a547fab0b4d38
   3:    std::panicking::default_hook::h7fba1b2f69474bdc
   4:    std::panicking::rust_panic_with_hook::h5d3597668c9f0035
   5:    std::panicking::begin_panic::hfaa38fdbc7d103f6
   6:    std::panicking::begin_panic_fmt::h69e0397ed8fc5362
   7:    rust_begin_unwind
   8:    core::panicking::panic_fmt::h2ba266031787cf7b
   9:    core::result::unwrap_failed::h0a78b8b4914ea99c
             at /home/yamakaky/dev/rust/rust/src/libcore/macros.rs:29
  10:    <core::result::Result<T, E>>::unwrap::h2f7c43c0e2d4b7cd
             at /home/yamakaky/dev/rust/rust/src/libcore/result.rs:737
  11:    t::main::h86d9b74d94eb7fff
             at /tmp/pote/t.rs:2
  12:    __rust_maybe_catch_panic
  13:    std::rt::lang_start::ha33d35ddc5bfd7d2
  14:    main
  15:    __libc_start_main
  16:    _start
