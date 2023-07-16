
Core file '/cores/core.33216' (x86_64) was loaded.
(lldb) command source -s 0 'cmds'
Executing commands in '/Users/travis/build/rust-lang/rust/cmds'.
(lldb) bt all
* thread #1: tid = 0x0000, 0x00007fffbb6ec19d libsystem_c.dylib`__cxa_finalize_ranges + 369, stop reason = signal SIGSTOP
    frame #0: 0x00007fffbb6ec19d libsystem_c.dylib`__cxa_finalize_ranges + 369
* thread #2: tid = 0x0001, 0x00007fffbb786756 libsystem_kernel.dylib`close + 10, stop reason = signal SIGSTOP
    frame #0: 0x00007fffbb786756 libsystem_kernel.dylib`close + 10
    frame #1: 0x0000000106869c10 ld`Snapshot::createSnapshot() + 270
    frame #2: 0x00000001067ac5da ld`__assert_rtn + 98
    frame #3: 0x00000001067fb3c4 ld`ld::tool::InputFiles::parseWorkerThread() + 696
    frame #4: 0x00007fffbb86eaab libsystem_pthread.dylib`_pthread_body + 180
    frame #5: 0x00007fffbb86e9f7 libsystem_pthread.dylib`_pthread_start + 286
    frame #6: 0x00007fffbb86e221 libsystem_pthread.dylib`thread_start + 13
