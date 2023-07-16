
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3 ~ lib[8787]::bar), const_param_did: None }) (end of phase Optimization) at bb1[1]:
encountered non-callable type impl Fn<()> in `Call` terminator
  --> Projects/rust/src/test/ui/issues/issue-50865-private-impl-trait/auxiliary/lib.rs:10:5
   |
10 |     hide_foo()();
   |     ^^^^^^^^^^^^
   |
  ::: /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:70:5
   |
70 |     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
   |     -------------------------------------------------------------- in the inlined copy of this code
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1012:13
stack backtrace:
   0:     0x7fa44379f930 - std::backtrace_rs::backtrace::libunwind::trace::hfe3b1cace85e87d8
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7fa44379f930 - std::backtrace_rs::backtrace::trace_unsynchronized::h542330af06479043
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fa44379f930 - std::sys_common::backtrace::_print_fmt::h6b88726367858985
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fa44379f930 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcd76ce6b485adbea
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fa44380dabf - core::fmt::write::h127419eb46f2ecc9
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/core/src/fmt/mod.rs:1092:17
   5:     0x7fa443793a42 - std::io::Write::write_fmt::h0facb64ef2e7f5d8
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/io/mod.rs:1567:15
   6:     0x7fa4437a3675 - std::sys_common::backtrace::_print::h7bf1633ea5421f7b
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fa4437a3675 - std::sys_common::backtrace::print::h5cc4d09049928ba5
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fa4437a3675 - std::panicking::default_hook::{{closure}}::h9e84dc005bfc9fc7
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/panicking.rs:208:50
   9:     0x7fa4437a31d3 - std::panicking::default_hook::h123278a03b1f796b
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/panicking.rs:225:9
  10:     0x7fa443f753fb - rustc_driver::report_ice::h036d06dbb7c2c24c
  11:     0x7fa4437a3de0 - std::panicking::rust_panic_with_hook::h4040631aa6c7bc27
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/panicking.rs:595:17
  12:     0x7fa4437a3957 - std::panicking::begin_panic_handler::{{closure}}::h02a17b22ac092d08
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/panicking.rs:497:13
  13:     0x7fa44379fdec - std::sys_common::backtrace::__rust_end_short_backtrace::h6063b024443b5852
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/sys_common/backtrace.rs:141:18
  14:     0x7fa4437a38b9 - rust_begin_unwind
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/panicking.rs:493:5
  15:     0x7fa44376803b - std::panicking::begin_panic_fmt::h10b51f96d6207ff1
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/panicking.rs:435:5
  16:     0x7fa44668486e - rustc_errors::HandlerInner::flush_delayed::hfd981d840ce2a87f
  17:     0x7fa44668317b - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hcc27ba916259eda8
  18:     0x7fa445cae396 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::hfd8ad55a7830aec9
  19:     0x7fa445cb0a10 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h60fefb93b4088059
  20:     0x7fa445cad56d - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h18cfca8587e69926
  21:     0x7fa445cacdda - rustc_span::with_source_map::h512fd197bcd2b7c3
  22:     0x7fa445cb1ee3 - scoped_tls::ScopedKey<T>::set::hd407f010a973eb63
  23:     0x7fa445cb33a4 - std::sys_common::backtrace::__rust_begin_short_backtrace::h3c42279179caa5db
  24:     0x7fa445cce765 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hd56f564efc764ea1
  25:     0x7fa4437b3258 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd8ea7ada17dfa868
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/alloc/src/boxed.rs:1546:9
  26:     0x7fa4437b3258 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h91ce3a636c58b978
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/alloc/src/boxed.rs:1546:9
  27:     0x7fa4437b3258 - std::sys::unix::thread::Thread::new::thread_start::h2e193c2e23720fdf
                               at /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/library/std/src/sys/unix/thread.rs:71:17
  28:     0x7fa4436e1609 - start_thread
  29:     0x7fa4435f5293 - clone
  30:                0x0 - <unknown>

error: internal compiler error: unexpected panic
