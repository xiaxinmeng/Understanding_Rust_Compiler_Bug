
uncaptured: disabled backtrace
captured: Backtrace [{ fn: std::backtrace::Backtrace::create::ha19bbd9f10287f80 }, { fn: std::backtrace::Backtrace::force_capture::h5e8065a48ff91123 }, { fn: std::backtrace::tests::debug_backtrace_fmt::h0557493242cdd27b }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h94c20d903b9ee892 }, { fn: __rust_maybe_catch_panic }, { fn: std::panicking::try::hc8e0ebad5360bb9e }, { fn: test::run_test_in_process::h23ee0e24f420835e }, { fn: std::sys_common::backtrace::__rust_begin_short_backtrace::h66d96ab05a49a4cf }, { fn: std::panicking::try::do_call::he44bd66b51853864 }, { fn: __rust_maybe_catch_panic }, { fn: std::panicking::try::h9e6571690ab8a505 }, { fn: core::ops::function::FnOnce::call_once{{vtable.shim}}::hb78017e7280200c9 }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h6caad6f61b1e39b2 }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hc9d5f44518fec1bf }, { fn: std::sys_common::thread::start_thread::h44e11c2292c6e0da }, { fn: std::sys::unix::thread::Thread::new::thread_start::h1534b513264a5d02 }, { fn: start_thread }, { fn: __clone }]
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

resolved: Backtrace [{ fn: std::backtrace::Backtrace::create::ha19bbd9f10287f80 }, { fn: std::backtrace::Backtrace::force_capture::h5e8065a48ff91123 }, { fn: std::backtrace::tests::debug_backtrace_fmt::h0557493242cdd27b }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h94c20d903b9ee892 }, { fn: __rust_maybe_catch_panic }, { fn: std::panicking::try::hc8e0ebad5360bb9e }, { fn: test::run_test_in_process::h23ee0e24f420835e }, { fn: std::sys_common::backtrace::__rust_begin_short_backtrace::h66d96ab05a49a4cf }, { fn: std::panicking::try::do_call::he44bd66b51853864 }, { fn: __rust_maybe_catch_panic }, { fn: std::panicking::try::h9e6571690ab8a505 }, { fn: core::ops::function::FnOnce::call_once{{vtable.shim}}::hb78017e7280200c9 }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h6caad6f61b1e39b2 }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hc9d5f44518fec1bf }, { fn: std::sys_common::thread::start_thread::h44e11c2292c6e0da }, { fn: std::sys::unix::thread::Thread::new::thread_start::h1534b513264a5d02 }, { fn: start_thread }, { fn: __clone }]
resolved alt: Backtrace [
    { fn: std::backtrace::Backtrace::create },
    { fn: std::backtrace::Backtrace::force_capture },
    { fn: std::backtrace::tests::debug_backtrace_fmt },
    { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once },
    { fn: __rust_maybe_catch_panic },
    { fn: std::panicking::try },
    { fn: test::run_test_in_process },
    { fn: std::sys_common::backtrace::__rust_begin_short_backtrace },
    { fn: std::panicking::try::do_call },
    { fn: __rust_maybe_catch_panic },
    { fn: std::panicking::try },
    { fn: core::ops::function::FnOnce::call_once{{vtable.shim}} },
    { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once },
    { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once },
    { fn: std::sys_common::thread::start_thread },
    { fn: std::sys::unix::thread::Thread::new::thread_start },
    { fn: start_thread },
    { fn: __clone },
]
