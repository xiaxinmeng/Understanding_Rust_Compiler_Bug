
uncaptured: disabled backtrace
captured: Backtrace [{ fn: "std::backtrace::tests::debug_backtrace_fmt" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "__rust_maybe_catch_panic" }, { fn: "std::panicking::try" }, { fn: "test::run_test_in_process" }, { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace" }, { fn: "std::panicking::try::do_call" }, { fn: "__rust_maybe_catch_panic" }, { fn: "std::panicking::try" }, { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "std::sys_common::thread::start_thread" }, { fn: "std::sys::unix::thread::Thread::new::thread_start" }, { fn: "start_thread" }, { fn: "__clone" }]
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
   0:     0x55ed655a1ebd - std::backtrace::Backtrace::create::ha19bbd9f10287f80
   1:     0x55ed655a1e30 - std::backtrace::Backtrace::force_capture::h5e8065a48ff91123
   2:     0x55ed656816f4 - std::backtrace::tests::debug_backtrace_fmt::h0557493242cdd27b
   3:     0x7f729e67028f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h94c20d903b9ee892
   4:     0x7f729e3cc844 - __rust_maybe_catch_panic
   5:     0x7f729e67247c - std::panicking::try::hc8e0ebad5360bb9e
   6:     0x7f729e67b2ef - test::run_test_in_process::h23ee0e24f420835e
   7:     0x7f729e6783e8 - std::sys_common::backtrace::__rust_begin_short_backtrace::h66d96ab05a49a4cf
   8:     0x7f729e6725a6 - std::panicking::try::do_call::he44bd66b51853864
   9:     0x7f729e3cc844 - __rust_maybe_catch_panic
  10:     0x7f729e67233a - std::panicking::try::h9e6571690ab8a505
  11:     0x7f729e672aa3 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb78017e7280200c9
  12:     0x7f729e39e6bf - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h6caad6f61b1e39b2
  13:     0x7f729e39e713 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hc9d5f44518fec1bf
  14:     0x7f729e3c1baa - std::sys_common::thread::start_thread::h44e11c2292c6e0da
  15:     0x7f729e3c7796 - std::sys::unix::thread::Thread::new::thread_start::h1534b513264a5d02
  16:     0x7f729e8dc6db - start_thread
  17:     0x7f729de5e88f - __clone
  18:                0x0 - <unknown>

resolved: Backtrace [{ fn: "std::backtrace::tests::debug_backtrace_fmt" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "__rust_maybe_catch_panic" }, { fn: "std::panicking::try" }, { fn: "test::run_test_in_process" }, { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace" }, { fn: "std::panicking::try::do_call" }, { fn: "__rust_maybe_catch_panic" }, { fn: "std::panicking::try" }, { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "std::sys_common::thread::start_thread" }, { fn: "std::sys::unix::thread::Thread::new::thread_start" }, { fn: "start_thread" }, { fn: "__clone" }]
resolved alt: Backtrace [
    { fn: "std::backtrace::tests::debug_backtrace_fmt" },
    { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" },
    { fn: "__rust_maybe_catch_panic" },
    { fn: "std::panicking::try" },
    { fn: "test::run_test_in_process" },
    { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace" },
    { fn: "std::panicking::try::do_call" },
    { fn: "__rust_maybe_catch_panic" },
    { fn: "std::panicking::try" },
    { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}" },
    { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" },
    { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" },
    { fn: "std::sys_common::thread::start_thread" },
    { fn: "std::sys::unix::thread::Thread::new::thread_start" },
    { fn: "start_thread" },
    { fn: "__clone" },
]
