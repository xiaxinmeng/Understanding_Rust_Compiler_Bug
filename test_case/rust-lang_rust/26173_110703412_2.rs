
(lldb) bt
* thread #1: tid = 0xeeeaff, 0x00007fff9144d7da libsystem_kernel.dylib`__bsdthread_create + 10, queue = 'com.apple.main-thread', stop reason = signal SIGPIPE
  * frame #0: 0x00007fff9144d7da libsystem_kernel.dylib`__bsdthread_create + 10
    frame #1: 0x00007fff8984a01d libsystem_pthread.dylib`pthread_create + 230
    frame #2: 0x000000010062fcdb libstd-d8ace771.dylib`std::sys::thread::Thread::new(stack=<unavailable>, p=<unavailable>) + 475 at thread.rs:64
    frame #3: 0x000000010042f7be libtest-d8ace771.dylib`test::thread::Builder::spawn_inner<()>(self=<unavailable>, f=<unavailable>) + 622 at mod.rs:353
    frame #4: 0x000000010042efd7 libtest-d8ace771.dylib`test::run_test::run_test_inner [inlined] test::thread::Builder::spawn<closure,()>(f=<unavailable>) + 26 at mod.rs:278
    frame #5: 0x000000010042efbd libtest-d8ace771.dylib`test::run_test::run_test_inner [inlined] test::thread::spawn<closure,()>(f=<unavailable>) + 442 at mod.rs:382
    frame #6: 0x000000010042ee03 libtest-d8ace771.dylib`test::run_test::run_test_inner(desc=<unavailable>, monitor_ch=<unavailable>, nocapture=<unavailable>, testfn=<unavailable>) + 499 at lib.rs:944
    frame #7: 0x000000010041f460 libtest-d8ace771.dylib`test::run_test(opts=<unavailable>, force_ignore=<unavailable>, test=<unavailable>, monitor_ch=<unavailable>) + 672 at lib.rs:989
    frame #8: 0x000000010040e6fe libtest-d8ace771.dylib`test::run_tests_console + 69 at lib.rs:821
    frame #9: 0x000000010040e6b9 libtest-d8ace771.dylib`test::run_tests_console(opts=0x00007fff5fbff0b0, tests=<unavailable>) + 7657 at lib.rs:698
    frame #10: 0x000000010040b573 libtest-d8ace771.dylib`test::test_main(args=<unavailable>, tests=<unavailable>) + 307 at lib.rs:253
    frame #11: 0x0000000100412d5e libtest-d8ace771.dylib`test::test_main_static(args=<unavailable>, tests=<unavailable>) + 2750 at lib.rs:276
    frame #12: 0x000000010022d1dc stdtest-x86_64-apple-darwin`__test::main::hd5f7e0a7fd4698957lw + 92
    frame #13: 0x00000001006e3939 libstd-d8ace771.dylib`rust_try_inner + 9
    frame #14: 0x00000001006e3926 libstd-d8ace771.dylib`rust_try + 6
    frame #15: 0x000000010064ae56 libstd-d8ace771.dylib`std::rt::lang_start [inlined] std::rt::unwind::try::inner_try(f=<unavailable>) + 76 at mod.rs:147
    frame #16: 0x000000010064ae0a libstd-d8ace771.dylib`std::rt::lang_start [inlined] std::rt::unwind::try<closure> at mod.rs:130
    frame #17: 0x000000010064ae0a libstd-d8ace771.dylib`std::rt::lang_start(main=<unavailable>, argc=<unavailable>, argv=<unavailable>) + 666 at mod.rs:128
    frame #18: 0x00007fff91f485c9 libdyld.dylib`start + 1
(lldb) 
