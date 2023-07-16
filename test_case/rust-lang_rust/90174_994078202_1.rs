
Error: Error with two sources

Caused by:
    0: The source of the error
    1: Error with backtrace

Stack backtrace:
   0: std::error::tests::error_with_backtrace_outputs_correctly_with_two_sources
   1: test::__rust_begin_short_backtrace
   2: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
   3: std::panicking::try
   4: test::run_test_in_process
   5: std::sys_common::backtrace::__rust_begin_short_backtrace
   6: std::panicking::try
   7: core::ops::function::FnOnce::call_once{{vtable.shim}}
   8: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
   9: std::sys::unix::thread::Thread::new::thread_start
  10: start_thread
             at /build/glibc-eX1tMB/glibc-2.31/nptl/pthread_create.c:477:8
