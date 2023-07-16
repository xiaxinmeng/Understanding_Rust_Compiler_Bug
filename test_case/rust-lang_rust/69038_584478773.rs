
display print: stack backtrace:
   0: std::backtrace::tests::debug_backtrace_fmt
   1: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
   2: __rust_maybe_catch_panic
   3: std::panicking::try
   4: test::run_test_in_process
   5: std::sys_common::backtrace::__rust_begin_short_backtrace
   6: std::panicking::try::do_call
   7: __rust_maybe_catch_panic
   8: std::panicking::try
   9: core::ops::function::FnOnce::call_once{{vtable.shim}}
  10: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
  11: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
  12: std::sys_common::thread::start_thread
  13: std::sys::unix::thread::Thread::new::thread_start
  14: start_thread
  15: __clone

display print alt: stack backtrace:
   0:     0x561d29648ead - std::backtrace::Backtrace::create::ha19bbd9f10287f80
   1:     0x561d29648e20 - std::backtrace::Backtrace::force_capture::h5e8065a48ff91123
   2:     0x561d297286e4 - std::backtrace::tests::debug_backtrace_fmt::h0557493242cdd27b
   3:     0x7fef1438528f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h94c20d903b9ee892
   4:     0x7fef140e1834 - __rust_maybe_catch_panic
   5:     0x7fef1438747c - std::panicking::try::hc8e0ebad5360bb9e
   6:     0x7fef143902ef - test::run_test_in_process::h23ee0e24f420835e
   7:     0x7fef1438d3e8 - std::sys_common::backtrace::__rust_begin_short_backtrace::h66d96ab05a49a4cf
   8:     0x7fef143875a6 - std::panicking::try::do_call::he44bd66b51853864
   9:     0x7fef140e1834 - __rust_maybe_catch_panic
  10:     0x7fef1438733a - std::panicking::try::h9e6571690ab8a505
  11:     0x7fef14387aa3 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb78017e7280200c9
  12:     0x7fef140b36af - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h6caad6f61b1e39b2
  13:     0x7fef140b3703 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hc9d5f44518fec1bf
  14:     0x7fef140d6baa - std::sys_common::thread::start_thread::h44e11c2292c6e0da
  15:     0x7fef140dc796 - std::sys::unix::thread::Thread::new::thread_start::h1534b513264a5d02
  16:     0x7fef145f16db - start_thread
  17:     0x7fef13b7388f - __clone
  18:                0x0 - <unknown>
