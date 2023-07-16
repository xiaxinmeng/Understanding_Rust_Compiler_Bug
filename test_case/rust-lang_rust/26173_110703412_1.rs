
(lldb) bt
* thread #1: tid = 0xeed4a1, 0x00007fff914495c2 libsystem_kernel.dylib`swtch_pri + 10, queue = 'com.apple.main-thread', stop reason = signal SIGPIPE
  * frame #0: 0x00007fff914495c2 libsystem_kernel.dylib`swtch_pri + 10
    frame #1: 0x00007fff8984b37a libsystem_pthread.dylib`_pthread_find_thread + 81
    frame #2: 0x00007fff8984b978 libsystem_pthread.dylib`_pthread_lookup_thread + 53
    frame #3: 0x00007fff8984b8d4 libsystem_pthread.dylib`pthread_detach + 22
    frame #4: 0x000000010062e715 libstd-d8ace771.dylib`std::sys::thread::Thread.Drop::drop(self=<unavailable>) + 53 at thread.rs:157
    frame #5: 0x000000010042f408 libtest-d8ace771.dylib`std..thread..JoinInner$LT$$LP$$RP$$GT$::drop.9367::ha226051b3a20aed3 + 72
    frame #6: 0x000000010042f1c0 libtest-d8ace771.dylib`test::run_test::run_test_inner(desc=<unavailable>, monitor_ch=<unavailable>, nocapture=<unavailable>, testfn=<unavailable>) + 1456 at lib.rs:944
    frame #7: 0x000000010041f460 libtest-d8ace771.dylib`test::run_test(opts=<unavailable>, force_ignore=<unavailable>, test=<unavailable>, monitor_ch=<unavailable>) + 672 at lib.rs:989
    frame #8: 0x000000010040e6fe libtest-d8ace771.dylib`test::run_tests_console + 69 at lib.rs:821
    frame #9: 0x000000010040e6b9 libtest-d8ace771.dylib`test::run_tests_console(opts=0x00007fff5fbff0b0, tests=<unavailable>) + 7657 at lib.rs:698
