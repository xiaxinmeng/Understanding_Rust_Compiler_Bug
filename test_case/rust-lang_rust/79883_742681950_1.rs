
(lldb) run
Process 21550 launched: '/Users/coreyf/dev/rust-lang/rust/badfree' (arm64)
==21550==ERROR: AddressSanitizer failed to allocate 0x200004000 (8589950976) bytes at address fffffc000 (errno: 12)
==21550==ReserveShadowMemoryRange failed while trying to map 0x200004000 bytes. Perhaps you're using ulimit -v
Process 21550 stopped
* thread #1, stop reason = signal SIGABRT
    frame #0: 0x000000018be3fcec libsystem_kernel.dylib`__pthread_kill + 8
libsystem_kernel.dylib`__pthread_kill:
->  0x18be3fcec <+8>:  b.lo   0x18be3fd0c               ; <+40>
    0x18be3fcf0 <+12>: pacibsp 
    0x18be3fcf4 <+16>: stp    x29, x30, [sp, #-0x10]!
    0x18be3fcf8 <+20>: mov    x29, sp
Target 0: (badfree) stopped.
