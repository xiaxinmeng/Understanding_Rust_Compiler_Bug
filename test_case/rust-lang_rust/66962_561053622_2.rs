text
error: internal compiler error: constant in type had an ignored error: TooGeneric
  --> src/main.rs:12:5
   |
12 |     arr: [u8; {CFG.0}]
   |     ^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:347:17
stack backtrace:
   0:     0x7f63673e75d4 - backtrace::backtrace::libunwind::trace::hc586f95f659e6084
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f63673e75d4 - backtrace::backtrace::trace_unsynchronized::ha9827fdb593fd967
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f63673e75d4 - std::sys_common::backtrace::_print_fmt::h00c888c95e07165a
                               at src/libstd/sys_common/backtrace.rs:84
   3:     0x7f63673e75d4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8407ffb2d059bc74
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7f636741fcec - core::fmt::write::h4165a12a3856465f
                               at src/libcore/fmt/mod.rs:1024
   5:     0x7f63673db937 - std::io::Write::write_fmt::h499a0566ceaa0048
                               at src/libstd/io/mod.rs:1428
   6:     0x7f63673eba7e - std::sys_common::backtrace::_print::h05fbb11587298e2b
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7f63673eba7e - std::sys_common::backtrace::print::h8021a3ed2b5ff07e
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7f63673eba7e - std::panicking::default_hook::{{closure}}::hd3a6326f5c6c149f
                               at src/libstd/panicking.rs:193
   9:     0x7f63673eb771 - std::panicking::default_hook::h7088fb00a0cb1faf
                               at src/libstd/panicking.rs:210
  10:     0x7f6367931833 - rustc_driver::report_ice::h7d5c1d6a7c4e3fb5
  11:     0x7f63673ec230 - std::panicking::rust_panic_with_hook::h6b223bff7721d4c1
                               at src/libstd/panicking.rs:475
  12:     0x7f63697c94f5 - std::panicking::begin_panic::h372a91a0f5e0855e
  13:     0x7f63697faa5c - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h222594a5efabe6ee
  14:     0x7f63678ddd96 - core::ptr::real_drop_in_place::ha878bee8c6c3164e
  15:     0x7f63678e3843 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h78a9ef975e0151c9
  16:     0x7f636790431c - core::ptr::real_drop_in_place::h126176f0f6835d28
  17:     0x7f63678f8860 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h450b3e47e99b2a02
  18:     0x7f63678c8b72 - std::thread::local::LocalKey<T>::with::hc064f9c463ba87b1
  19:     0x7f63678c268e - scoped_tls::ScopedKey<T>::set::he618e0867346e071
  20:     0x7f636793c9a4 - syntax::with_globals::h05656a832d1edc4c
  21:     0x7f636793eb00 - std::sys_common::backtrace::__rust_begin_short_backtrace::hb45a6b809630e3e0
  22:     0x7f63673fcd0a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:78
  23:     0x7f63678d9469 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h0091d3870173b469
  24:     0x7f63673cd88f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h9ef9eb6ec2ee6be0
                               at /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/liballoc/boxed.rs:969
  25:     0x7f63673fb730 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hc0fc46e9a64f076e
                               at /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/liballoc/boxed.rs:969
  26:     0x7f63673fb730 - std::sys_common::thread::start_thread::h4eee21a391e25c99
                               at src/libstd/sys_common/thread.rs:13
  27:     0x7f63673fb730 - std::sys::unix::thread::Thread::new::thread_start::h673f7c20aae94594
                               at src/libstd/sys/unix/thread.rs:80
  28:     0x7f636733a669 - start_thread
  29:     0x7f6367251323 - clone
  30:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (fdc001156 2019-12-02) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
