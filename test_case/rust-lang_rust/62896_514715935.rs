
GNU gdb (Raspbian 8.2.1-2) 8.2.1
Copyright (C) 2018 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "arm-linux-gnueabihf".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from cargo...done.
(gdb) r
Starting program: /home/fenhl/.cargo/bin/cargo check --release
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
process 16341 is executing new program: /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/cargo
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
[New Thread 0x76d512f0 (LWP 16344)]
   Compiling libc v0.2.60
[New Thread 0x769fe2f0 (LWP 16345)]
[Detaching after fork from child process 16346]                          ] 0/4: libc(build.rs)                                                                                              
[Thread 0x769fe2f0 (LWP 16345) exited]

Thread 2 "cargo" received signal SIGUSR1, User defined signal 1.
[Switching to Thread 0x76d512f0 (LWP 16344)]
futex_wait_cancelable (private=0, expected=0, futex_word=0xe77938) at ../sysdeps/unix/sysv/linux/futex-internal.h:88
88	../sysdeps/unix/sysv/linux/futex-internal.h: No such file or directory.
(gdb) bt
#0  futex_wait_cancelable (private=0, expected=0, futex_word=0xe77938) at ../sysdeps/unix/sysv/linux/futex-internal.h:88
#1  __pthread_cond_wait_common (abstime=0x0, mutex=0x0, cond=0xe77910) at pthread_cond_wait.c:502
#2  __pthread_cond_wait (cond=0xe77910, mutex=0x0) at pthread_cond_wait.c:655
#3  0x00bd0900 in std::sys::unix::condvar::Condvar::wait () at src/libstd/sys/unix/condvar.rs:69
#4  std::sys_common::condvar::Condvar::wait () at src/libstd/sys_common/condvar.rs:41
#5  std::sync::condvar::Condvar::wait () at src/libstd/sync/condvar.rs:204
#6  std::thread::park () at src/libstd/thread/mod.rs:911
#7  0x00bde4b4 in std::sync::mpsc::blocking::WaitToken::wait () at src/libstd/sync/mpsc/blocking.rs:71
#8  0x00baf508 in std::sync::mpsc::oneshot::Packet<T>::recv ()
#9  0x00baddb8 in std::sync::mpsc::Receiver<T>::recv ()
#10 0x00bb2d38 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
#11 0x00bb1b44 in std::panicking::try::do_call ()
#12 0x00bebf88 in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:82
#13 0x00bb2110 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
#14 0x00bcfb74 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/a7f28678bbf4e16893bb6a718e427504167a9494/src/liballoc/boxed.rs:766
#15 0x00beb48c in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/a7f28678bbf4e16893bb6a718e427504167a9494/src/liballoc/boxed.rs:766
#16 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#17 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#18 0x76f56494 in start_thread (arg=0x76d512f0) at pthread_create.c:486
Backtrace stopped: Cannot access memory at address 0x1ec9a
(gdb) q
A debugging session is active.

	Inferior 1 [process 16341] will be killed.

Quit anyway? (y or n) y
