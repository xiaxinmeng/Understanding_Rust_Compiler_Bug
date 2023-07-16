
error: internal compiler error: impl item and trait item have different parameter counts
  |
  = note: delayed at /rustc/346aec9b02f3c74f3fce97fd6bda24709d220e49/src/librustc_session/session.rs:436:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/346aec9b02f3c74f3fce97fd6bda24709d220e49/src/librustc_session/session.rs:436:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:367:17
stack backtrace:
   0:     0x7f5c742468a5 - backtrace::backtrace::libunwind::trace::h34afbfad7fd770fc
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x7f5c742468a5 - backtrace::backtrace::trace_unsynchronized::h460d522b1619a600
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x7f5c742468a5 - std::sys_common::backtrace::_print_fmt::ha45fac10086813b4
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x7f5c742468a5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde84f63fcfd0e6de
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f5c7428332c - core::fmt::write::h540ac4a6a1232abc
                               at src/libcore/fmt/mod.rs:1076
   5:     0x7f5c74238622 - std::io::Write::write_fmt::hc344eafd6e850b4d
                               at src/libstd/io/mod.rs:1537
   6:     0x7f5c7424b740 - std::sys_common::backtrace::_print::h4db88ff15cb5d61d
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f5c7424b740 - std::sys_common::backtrace::print::h5fc39e1b1f610bd3
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f5c7424b740 - std::panicking::default_hook::{{closure}}::h59e55edacb1d974a
                               at src/libstd/panicking.rs:198
   9:     0x7f5c7424b48c - std::panicking::default_hook::heee4c8016dfbf328
                               at src/libstd/panicking.rs:217
  10:     0x7f5c749b6489 - rustc_driver::report_ice::h779cc1b2c44c51e9
  11:     0x7f5c7424beb8 - std::panicking::rust_panic_with_hook::h8405b6301c79fb5a
                               at src/libstd/panicking.rs:530
  12:     0x7f5c77503bde - std::panicking::begin_panic::h6ea7578d4a96992b
  13:     0x7f5c77537892 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hbf11e6477c5496bc
  14:     0x7f5c749fc746 - core::ptr::drop_in_place::hf14317e84460e945
  15:     0x7f5c749ffdf6 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::hc7ca667378a494ce
  16:     0x7f5c74a145fd - core::ptr::drop_in_place::h04ba5b058b45842d
  17:     0x7f5c74a0ffeb - rustc_span::with_source_map::h276db6bb3fd85930
  18:     0x7f5c74971ae6 - rustc_interface::interface::create_compiler_and_run::h7654f3523c3fe4ef
  19:     0x7f5c7499c35d - scoped_tls::ScopedKey<T>::set::hec735e67099211f0
  20:     0x7f5c749c2542 - std::sys_common::backtrace::__rust_begin_short_backtrace::hc623bbd3fa27b625
  21:     0x7f5c7497f88e - core::ops::function::FnOnce::call_once{{vtable.shim}}::he5d650d3d699089a
  22:     0x7f5c7425ac7a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h13d34828db364579
                               at /rustc/346aec9b02f3c74f3fce97fd6bda24709d220e49/src/liballoc/boxed.rs:1081
  23:     0x7f5c7425ac7a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd51b619e0f884abf
                               at /rustc/346aec9b02f3c74f3fce97fd6bda24709d220e49/src/liballoc/boxed.rs:1081
  24:     0x7f5c7425ac7a - std::sys::unix::thread::Thread::new::thread_start::h02c6e34c2c73f344
                               at src/libstd/sys/unix/thread.rs:87
  25:     0x7f5c74181422 - start_thread
  26:     0x7f5c7409fbf3 - __GI___clone
  27:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-nightly (346aec9b0 2020-07-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
