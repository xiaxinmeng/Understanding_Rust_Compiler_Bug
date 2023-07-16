
* thread #2, name = 'index::btree::test::large_page', stop reason = EXC_BAD_ACCESS (code=2, address=0x700005aec840)
  * frame #0: 0x000000010ed21f93 neb-96fd92871c91115d`__rust_probestack at probestack.rs:55 [opt]
    frame #1: 0x000000010e404f01 neb-96fd92871c91115d`neb::index::btree::test::large_page::_$u7b$$u7b$closure$u7d$$u7d$::h889a369a7e721f35 + 17
    frame #2: 0x000000010e2466a1 neb-96fd92871c91115d`core::ops::function::FnOnce::call_once::h97c0f1df1e597879 + 17
    frame #3: 0x000000010e483602 neb-96fd92871c91115d`call_box<(),closure> [inlined] {{closure}} at lib.rs:1474 [opt]
    frame #4: 0x000000010e4835fd neb-96fd92871c91115d`call_box<(),closure> [inlined] call_once<closure,()> at function.rs:231 [opt]
    frame #5: 0x000000010e4835fd neb-96fd92871c91115d`call_box<(),closure> at boxed.rs:734 [opt]
    frame #6: 0x000000010ed0783f neb-96fd92871c91115d`__rust_maybe_catch_panic at lib.rs:92 [opt]
    frame #7: 0x000000010e49f417 neb-96fd92871c91115d`{{closure}} [inlined] try<(),std::panic::AssertUnwindSafe<alloc::boxed::Box<FnBox<()>>>> at panicking.rs:276 [opt]
    frame #8: 0x000000010e49f3d2 neb-96fd92871c91115d`{{closure}} [inlined] catch_unwind<std::panic::AssertUnwindSafe<alloc::boxed::Box<FnBox<()>>>,()> at panic.rs:388 [opt]
    frame #9: 0x000000010e49f3d2 neb-96fd92871c91115d`{{closure}} at lib.rs:1429 [opt]
    frame #10: 0x000000010e47bfc5 neb-96fd92871c91115d`__rust_begin_short_backtrace<closure,()> at backtrace.rs:135 [opt]
    frame #11: 0x000000010e47c5e5 neb-96fd92871c91115d`do_call<std::panic::AssertUnwindSafe<closure>,()> [inlined] {{closure}}<closure,()> at mod.rs:469 [opt]
    frame #12: 0x000000010e47c5d2 neb-96fd92871c91115d`do_call<std::panic::AssertUnwindSafe<closure>,()> [inlined] call_once<(),closure> at panic.rs:309 [opt]
    frame #13: 0x000000010e47c5d2 neb-96fd92871c91115d`do_call<std::panic::AssertUnwindSafe<closure>,()> at panicking.rs:297 [opt]
    frame #14: 0x000000010ed0783f neb-96fd92871c91115d`__rust_maybe_catch_panic at lib.rs:92 [opt]
    frame #15: 0x000000010e4837f5 neb-96fd92871c91115d`call_box<(),closure> [inlined] try<(),std::panic::AssertUnwindSafe<closure>> at panicking.rs:276 [opt]
    frame #16: 0x000000010e4837bc neb-96fd92871c91115d`call_box<(),closure> [inlined] catch_unwind<std::panic::AssertUnwindSafe<closure>,()> at panic.rs:388 [opt]
    frame #17: 0x000000010e4837bc neb-96fd92871c91115d`call_box<(),closure> [inlined] {{closure}}<closure,()> at mod.rs:468 [opt]
    frame #18: 0x000000010e48377e neb-96fd92871c91115d`call_box<(),closure> at boxed.rs:734 [opt]
    frame #19: 0x000000010ed06e1c neb-96fd92871c91115d`thread_start [inlined] call_once<(),()> at boxed.rs:744 [opt]
    frame #20: 0x000000010ed06e19 neb-96fd92871c91115d`thread_start [inlined] start_thread at thread.rs:14 [opt]
    frame #21: 0x000000010ed06d9e neb-96fd92871c91115d`thread_start at thread.rs:81 [opt]
    frame #22: 0x00007fff5e628305 libsystem_pthread.dylib`_pthread_body + 126
    frame #23: 0x00007fff5e62b26f libsystem_pthread.dylib`_pthread_start + 70
    frame #24: 0x00007fff5e627415 libsystem_pthread.dylib`thread_start + 13
