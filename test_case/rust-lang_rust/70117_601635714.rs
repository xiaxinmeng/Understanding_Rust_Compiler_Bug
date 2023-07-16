
#0  0x00007ffff7f3dcf5 in pthread_cond_wait@@GLIBC_2.3.2 () from /usr/lib/libpthread.so.0
#1  0x0000555555f45e0e in jobserver::HelperState::for_each_request ()
#2  0x0000555555f461dc in std::sys_common::backtrace::__rust_begin_short_backtrace ()
#3  0x0000555555f46d38 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
#4  0x0000555555f5bd8f in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/f509b26a7730d721ef87423a72b3fdf8724b4afa/src/liballoc/boxed.rs:1017
#5  0x0000555555f7e70d in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/f509b26a7730d721ef87423a72b3fdf8724b4afa/src/liballoc/boxed.rs:1017
#6  std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#7  std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:80
#8  0x00007ffff7f3746f in start_thread () from /usr/lib/libpthread.so.0
#9  0x00007ffff7e4d3d3 in clone () from /usr/lib/libc.so.6
