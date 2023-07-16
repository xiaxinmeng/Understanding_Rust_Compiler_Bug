
   Compiling diesel v1.4.8
error: could not compile `cookie_store`

Caused by:
  process didn't exit successfully: `rustc --crate-name cookie_store --edition=2018 ........ --cap-lints allow` (signal: 6, SIGABRT: process abort signal)
warning: build failed, waiting for other jobs to finish...
Process 64899 stopped
* thread #5, stop reason = signal SIGUSR1
    frame #0: 0x00007ff80dc1da76 libsystem_pthread.dylib`_pthread_cond_wait + 1256
libsystem_pthread.dylib`_pthread_cond_wait:
->  0x7ff80dc1da76 <+1256>: callq  0x7ff80dc1a97e            ; pthread_testcancel
    0x7ff80dc1da7b <+1261>: leaq   -0x88(%rbp), %rax
    0x7ff80dc1da82 <+1268>: movq   0x10(%rax), %rax
    0x7ff80dc1da86 <+1272>: movq   %rax, 0x8(%rbx)
Target 0: (cargo) stopped.
(lldb)
Process 64899 resuming
Process 64899 stopped
* thread #5, stop reason = signal SIGUSR1
    frame #0: 0x00007ff80dc1da86 libsystem_pthread.dylib`_pthread_cond_wait + 1272
libsystem_pthread.dylib`_pthread_cond_wait:
->  0x7ff80dc1da86 <+1272>: movq   %rax, 0x8(%rbx)
    0x7ff80dc1da8a <+1276>: jmp    0x7ff80dc1daab            ; <+1309>
    0x7ff80dc1da8c <+1278>: movq   -0x58(%rbp), %rcx
    0x7ff80dc1da90 <+1282>: movq   -0x40(%rbp), %rdi
Target 0: (cargo) stopped.
