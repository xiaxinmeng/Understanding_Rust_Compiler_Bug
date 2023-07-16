shell
$ lldb segfault
(lldb) target create "segfault"
Current executable set to 'segfault' (x86_64).
(lldb) run
Process 57139 launched: '/Users/felipenoris/Documents/src/learnrust/segfault/target/debug/segfault' (x86_64)
segfault was compiled with optimization - stepping may behave oddly; variables may not be available.
Process 57139 stopped
* thread #1: tid = 0x793cbc, 0x0000000100011abf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] core::ptr::swap_nonoverlapping_bytes::h90af7d9428701cbf + 16 at ptr.rs:237, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x0000000100011abf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] core::ptr::swap_nonoverlapping_bytes::h90af7d9428701cbf + 16 at ptr.rs:237 [opt]
