
Process 12004 launched: '/Users/dradtke/Workspace/rust/dylib/main/target/debug/main' (x86_64)
Message: hello there world
Process 12004 stopped
* thread #1: tid = 0x78d98d, 0x00007fff9932dca9 libsystem_platform.dylib`OSSpinLockLock + 7, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x00007fff9932dca9 libsystem_platform.dylib`OSSpinLockLock + 7
libsystem_platform.dylib`OSSpinLockLock:
->  0x7fff9932dca9 <+7>:  lock
    0x7fff9932dcaa <+8>:  cmpxchgl %ecx, (%rdi)
    0x7fff9932dcad <+11>: jne    0x7fff9932dcb0            ; <+14>
    0x7fff9932dcaf <+13>: retq
