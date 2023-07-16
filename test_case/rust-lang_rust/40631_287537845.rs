
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:323
stack backtrace:
   0:     0x555a754c3ac3 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::hf9ed9ccfd9f14c2b
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x555a754bfdf4 - std::sys_common::backtrace::_print::hd8a1b72dcf3955ef
                               at /checkout/src/libstd/sys_common/backtrace.rs:71
   2:     0x555a754cb6bc - std::panicking::default_hook::{{closure}}::h5ff605bba7612658
                               at /checkout/src/libstd/sys_common/backtrace.rs:60
                               at /checkout/src/libstd/panicking.rs:355
   3:     0x555a754cb284 - std::panicking::default_hook::h9bc4f6dfee57d6bd
                               at /checkout/src/libstd/panicking.rs:371
   4:     0x555a754cbb0b - std::panicking::rust_panic_with_hook::hdc01585dc2bf7122
                               at /checkout/src/libstd/panicking.rs:549
   5:     0x555a754cb9e4 - std::panicking::begin_panic::hf84f4975d9f9b642
                               at /checkout/src/libstd/panicking.rs:511
   6:     0x555a754cb919 - std::panicking::begin_panic_fmt::hcc3f360b2ba80419
                               at /checkout/src/libstd/panicking.rs:495
   7:     0x555a754cb8a7 - rust_begin_unwind
                               at /checkout/src/libstd/panicking.rs:471
   8:     0x555a754f801d - core::panicking::panic_fmt::h795d9a9608ddc2bb
                               at /checkout/src/libcore/panicking.rs:69
   9:     0x555a754f7f54 - core::panicking::panic::hcab3e0dfa81beee9
                               at /checkout/src/libcore/panicking.rs:49
  10:     0x555a7517a6de - cargo::version::haea7e89a3b191e3c
                               at /checkout/src/libcore/macros.rs:21
                               at /checkout/cargo/src/cargo/lib.rs:263
  11:     0x555a74f2c7eb - cargo::execute::h38e6c27236d82ec3
                               at /checkout/cargo/src/bin/cargo.rs:156
  12:     0x555a74f24701 - cargo::call_main_without_stdin::hde98a839d8ab6c0f
                               at /checkout/cargo/src/cargo/lib.rs:128
  13:     0x555a74f2c2e1 - cargo::main::h2cff0d0817bf7ef6
                               at /checkout/cargo/src/bin/cargo.rs:91
                               at /checkout/cargo/src/bin/cargo.rs:84
  14:     0x555a754cb805 - std::panicking::try::do_call::h689a21caeeef92aa
                               at /checkout/src/libstd/panicking.rs:454
  15:     0x555a754d2aca - __rust_maybe_catch_panic
                               at /checkout/src/libpanic_unwind/lib.rs:98
  16:     0x555a754cc2a6 - std::rt::lang_start::hf63d494cb7dd034c
                               at /checkout/src/libstd/panicking.rs:433
                               at /checkout/src/libstd/panic.rs:361
                               at /checkout/src/libstd/rt.rs:57
  17:     0x7fb9e3e8f7ec - __libc_start_main
  18:     0x555a74f13b4c - <unknown>
