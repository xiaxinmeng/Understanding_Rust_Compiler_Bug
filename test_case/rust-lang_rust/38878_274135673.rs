
Core file '/cores/core.31933' (x86_64) was loaded.
(lldb) command source -s 0 'cmds'
Executing commands in '/Users/travis/build/rust-lang/rust/cmds'.
(lldb) bt all
* thread #1: tid = 0x0000, 0x00007fffaed8519d libsystem_c.dylib`__cxa_finalize_ranges + 369, stop reason = signal SIGSTOP
  * frame #0: 0x00007fffaed8519d libsystem_c.dylib`__cxa_finalize_ranges + 369

  thread #2: tid = 0x0001, 0x000000010f9fe5b4 dyld`ImageLoaderMachO::findClosestSymbol(mach_header const*, void const*, void const**) + 264, stop reason = signal SIGSTOP
    frame #0: 0x000000010f9fe5b4 dyld`ImageLoaderMachO::findClosestSymbol(mach_header const*, void const*, void const**) + 264
    frame #1: 0x000000010f9f5444 dyld`dladdr + 133
    frame #2: 0x00007fffaeced99c libdyld.dylib`dladdr + 72
    frame #3: 0x0000000100316647 ld`__assert_rtn + 207
    frame #4: 0x00000001003653c4 ld`ld::tool::InputFiles::parseWorkerThread() + 696
    frame #5: 0x00007fffaef07aab libsystem_pthread.dylib`_pthread_body + 180
    frame #6: 0x00007fffaef079f7 libsystem_pthread.dylib`_pthread_start + 286
    frame #7: 0x00007fffaef07221 libsystem_pthread.dylib`thread_start + 13
