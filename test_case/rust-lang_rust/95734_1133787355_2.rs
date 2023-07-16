
error: could not compile `png`

Caused by:
  process didn't exit successfully: `rustc --crate-name png --edition=2018 /home/build/.cargo/registry/src/github.com-1ecc6299db9ec823/png-0.16.8/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="deflate"' --cfg 'feature="png-encoding"' -C metadata=c95eabd95ff1a557 -C extra-filename=-c95eabd95ff1a557 --out-dir /home/build/test_build_crash/target/debug/deps -L dependency=/home/build/test_build_crash/target/debug/deps --extern bitflags=/home/build/test_build_crash/target/debug/deps/libbitflags-462dfa36d3305633.rmeta --extern crc32fast=/home/build/test_build_crash/target/debug/deps/libcrc32fast-88b102390994a92f.rmeta --extern deflate=/home/build/test_build_crash/target/debug/deps/libdeflate-8c45156971286026.rmeta --extern miniz_oxide=/home/build/test_build_crash/target/debug/deps/libminiz_oxide-0a8811d7c10dab74.rmeta --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...

...

Thread 2 "cargo" received signal SIGUSR1, User defined signal 1.
[Switching to Thread 0x7ffff4720700 (LWP 14139)]
0x00007ffffedbdad3 in futex_wait_cancelable (private=<optimized out>, expected=0, futex_word=0xa10aba8) at ../sysdeps/unix/sysv/linux/futex-internal.h:88
88      ../sysdeps/unix/sysv/linux/futex-internal.h: No such file or directory.
(gdb) bt
#0  0x00007ffffedbdad3 in futex_wait_cancelable (private=<optimized out>, expected=0, futex_word=0xa10aba8) at ../sysdeps/unix/sysv/linux/futex-internal.h:88
#1  __pthread_cond_wait_common (abstime=0x0, mutex=0xa12a750, cond=0xa10ab80) at pthread_cond_wait.c:502
#2  __pthread_cond_wait (cond=0xa10ab80, mutex=0xa12a750) at pthread_cond_wait.c:655
#3  0x0000000008b556f1 in _RINvMs3_CslUXF0WaltAi_9jobserverNtB6_11HelperState16for_each_requestNCNCNvNtB6_3imp12spawn_helpers_00EB6_ ()
#4  0x0000000008b576dc in _RINvNtNtCsjGQuiWR1kc_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvNtCslUXF0WaltAi_9jobserver3imp12spawn_helpers_0uEB1k_ ()
#5  0x0000000008b56daf in _RNSNvYNCINvMNtCsjGQuiWR1kc_3std6threadNtBa_7Builder16spawn_unchecked_NCNvNtCslUXF0WaltAi_9jobserver3imp12spawn_helpers_0uEs_0INtNtNtCse1161h7eFfp_4core3ops8function6FnOnceuE9call_once6vtableB1b_ ()
#6  0x0000000008b92053 in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/alloc/src/boxed.rs:1861
#7  <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/alloc/src/boxed.rs:1861
#8  std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:108
#9  0x00007ffffedb76db in start_thread (arg=0x7ffff4720700) at pthread_create.c:463
#10 0x00007ffffe52161f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
(gdb)
