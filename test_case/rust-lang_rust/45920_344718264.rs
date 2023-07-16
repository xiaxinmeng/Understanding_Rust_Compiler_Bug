
(gdb) where
#0  0xf7ea6dd0 in __rust_probestack () from /home/rustacean/rust/build/i686-unknown-linux-gnu/stage2/lib/librustc_driver-57160753fe603ca7.so
#1  0xf7db46be in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/rustacean/rust/build/i686-unknown-linux-gnu/stage2/lib/librustc_driver-57160753fe603ca7.so
#2  0xf7c67488 in __rust_maybe_catch_panic () from /home/rustacean/rust/build/i686-unknown-linux-gnu/stage2/lib/libstd-650e8e2b6b1595ab.so
#3  0xf7e36752 in <F as alloc::boxed::FnBox<A>>::call_box () from /home/rustacean/rust/build/i686-unknown-linux-gnu/stage2/lib/librustc_driver-57160753fe603ca7.so
#4  0xf7c0a635 in std::sys::imp::thread::Thread::new::thread_start () from /home/rustacean/rust/build/i686-unknown-linux-gnu/stage2/lib/libstd-650e8e2b6b1595ab.so
#5  0xf532b2f5 in start_thread (arg=0xf1dffb40) at pthread_create.c:456
#6  0xf7ac76de in clone () at ../sysdeps/unix/sysv/linux/i386/clone.S:113
