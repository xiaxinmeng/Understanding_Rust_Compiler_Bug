
(lldb) run
Process 48199 launched: './diceware-c7f6b180dd3b52c6' (x86_64)

running 3 tests
Process 48199 stopped
* thread #2: tid = 0x95c6e0, 0x00000001000cc9d2 diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a + 66 at ptr.rs:237, name = 'tests::returns_an_error_if_number_of_words_is_zero', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x00000001000cc9d2 diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a + 66 at ptr.rs:237
(lldb) bt
* thread #2: tid = 0x95c6e0, 0x00000001000cc9d2 diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a + 66 at ptr.rs:237, name = 'tests::returns_an_error_if_number_of_words_is_zero', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x00000001000cc9d2 diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a + 66 at ptr.rs:237
    frame #1: 0x000000010003dec3 diceware-c7f6b180dd3b52c6`std::sys_common::backtrace::__rust_begin_short_backtrace::hdb024ac408a215ad (.llvm.10125672080075898413) + 563 at mod.rs:650
    frame #2: 0x0000000100054f78 diceware-c7f6b180dd3b52c6`std::panicking::try::do_call::hf7d4215061b5619b (.llvm.18293538534938544574) + 40 at mod.rs:409
    frame #3: 0x00000001000d865f diceware-c7f6b180dd3b52c6`__rust_maybe_catch_panic + 31 at lib.rs:105
    frame #4: 0x000000010003f7b5 diceware-c7f6b180dd3b52c6`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hbf42c8a8f5d699cc + 165 at panicking.rs:289
    frame #5: 0x00000001000c9018 diceware-c7f6b180dd3b52c6`std::sys_common::thread::start_thread::hf39c8bd91f08cd93 + 136 at boxed.rs:648
    frame #6: 0x00000001000b95d9 diceware-c7f6b180dd3b52c6`std::sys::unix::thread::Thread::new::thread_start::h27c7af0fa85baf64 + 9 at thread.rs:90
    frame #7: 0x00007fff93c5299d libsystem_pthread.dylib`_pthread_body + 131
    frame #8: 0x00007fff93c5291a libsystem_pthread.dylib`_pthread_start + 168
    frame #9: 0x00007fff93c50351 libsystem_pthread.dylib`thread_start + 13
