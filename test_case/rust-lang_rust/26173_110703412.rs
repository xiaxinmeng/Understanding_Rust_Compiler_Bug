
(lldb) bt
* thread #1: tid = 0xeee85d, 0x00007fff9144e136 libsystem_kernel.dylib`__psynch_cvwait + 10, queue = 'com.apple.main-thread', stop reason = signal SIGPIPE
  * frame #0: 0x00007fff9144e136 libsystem_kernel.dylib`__psynch_cvwait + 10
    frame #1: 0x00007fff8984ae0c libsystem_pthread.dylib`_pthread_cond_wait + 693
    frame #2: 0x00000001005f7770 libstd-d8ace771.dylib`std::thread::park + 8 at condvar.rs:47
    frame #3: 0x00000001005f7768 libstd-d8ace771.dylib`std::thread::park [inlined] std::sys_common::condvar::Condvar::wait at condvar.rs:44
    frame #4: 0x00000001005f7768 libstd-d8ace771.dylib`std::thread::park [inlined] std::sync::condvar::StaticCondvar::wait<bool>(self=<unavailable>) + 17 at condvar.rs:255
    frame #5: 0x00000001005f7757 libstd-d8ace771.dylib`std::thread::park [inlined] std::sync::condvar::Condvar::wait<bool> + 32 at condvar.rs:133
    frame #6: 0x00000001005f7737 libstd-d8ace771.dylib`std::thread::park + 295 at mod.rs:517
    frame #7: 0x0000000100635bb2 libstd-d8ace771.dylib`std::sync::mpsc::blocking::WaitToken::wait(self=<unavailable>) + 82 at blocking.rs:83
    frame #8: 0x000000010042502e libtest-d8ace771.dylib`test::sync::mpsc::Receiver<T>::recv [inlined] test::sync::mpsc::shared::Packet<T>::recv(self=<unavailable>) + 65 at shared.rs:231
    frame #9: 0x0000000100424fed libtest-d8ace771.dylib`test::sync::mpsc::Receiver<T>::recv(self=<unavailable>) + 4365 at mod.rs:792
    frame #10: 0x000000010040e782 libtest-d8ace771.dylib`test::run_tests_console + 201 at lib.rs:825
    frame #11: 0x000000010040e6b9 libtest-d8ace771.dylib`test::run_tests_console(opts=0x00007fff5fbff0b0, tests=<unavailable>) + 7657 at lib.rs:698
