
* thread #2, name = 'test_1', stop reason = EXC_BAD_ACCESS (code=10, address=0x1001ef000)
  * frame #0: 0x0000000100055201 a-cfbe649daab6d4af`macho_get_commands + 145
    frame #1: 0x00000001000564cc a-cfbe649daab6d4af`macho_try_dwarf + 396
    frame #2: 0x0000000100056f86 a-cfbe649daab6d4af`macho_try_dsym + 726
    frame #3: 0x0000000100057519 a-cfbe649daab6d4af`macho_add + 1225
    frame #4: 0x0000000100057707 a-cfbe649daab6d4af`backtrace_initialize + 263
    frame #5: 0x0000000100054a9e a-cfbe649daab6d4af`fileline_initialize + 558
    frame #6: 0x0000000100054b7d a-cfbe649daab6d4af`backtrace_syminfo + 45
    frame #7: 0x000000010003fd52 a-cfbe649daab6d4af`core::slice::{{impl}}::position::{{closure}}<std::sys_common::backtrace::Frame,closure> [inlined] std::sys_common::gnu::libbacktrace::resolve_symname<closure> at libbacktrace.rs:79 [opt]
    frame #8: 0x000000010003fd1f a-cfbe649daab6d4af`core::slice::{{impl}}::position::{{closure}}<std::sys_common::backtrace::Frame,closure> [inlined] std::sys::imp::backtrace::printing::resolve_symname<closure> at mod.rs:36 [opt]
    frame #9: 0x000000010003fd1f a-cfbe649daab6d4af`core::slice::{{impl}}::position::{{closure}}<std::sys_common::backtrace::Frame,closure> [inlined] std::sys_common::backtrace::filter_frames::{{closure}} at backtrace.rs:108 [opt]
    frame #10: 0x000000010003fd18 a-cfbe649daab6d4af`core::slice::{{impl}}::position::{{closure}}<std::sys_common::backtrace::Frame,closure> at mod.rs:1204 [opt]
    frame #11: 0x0000000100047df1 a-cfbe649daab6d4af`std::sys_common::backtrace::_print [inlined] core::slice::{{impl}}::search_while<std::sys_common::backtrace::Frame,core::option::Option<usize>,closure> at mod.rs:1271 [opt]
    frame #12: 0x0000000100047daf a-cfbe649daab6d4af`std::sys_common::backtrace::_print [inlined] core::slice::{{impl}}::position<std::sys_common::backtrace::Frame,closure> at mod.rs:1203 [opt]
    frame #13: 0x0000000100047da3 a-cfbe649daab6d4af`std::sys_common::backtrace::_print [inlined] std::sys_common::backtrace::filter_frames at backtrace.rs:106 [opt]
    frame #14: 0x0000000100047d7f a-cfbe649daab6d4af`std::sys_common::backtrace::_print at backtrace.rs:71 [opt]
    frame #15: 0x000000010004d224 a-cfbe649daab6d4af`std::panicking::default_hook::{{closure}} [inlined] std::sys_common::backtrace::print at backtrace.rs:58 [opt]
    frame #16: 0x000000010004d203 a-cfbe649daab6d4af`std::panicking::default_hook::{{closure}} at panicking.rs:381 [opt]
    frame #17: 0x000000010004cd73 a-cfbe649daab6d4af`std::panicking::default_hook at panicking.rs:391 [opt]
    frame #18: 0x000000010004d703 a-cfbe649daab6d4af`std::panicking::rust_panic_with_hook at panicking.rs:577 [opt]
    frame #19: 0x0000000100000ffe a-cfbe649daab6d4af`std::panicking::begin_panic::hcb69775319227ce6 + 110
    frame #20: 0x0000000100000f26 a-cfbe649daab6d4af`a::test_1::h499e1b269bece65b + 38
    frame #21: 0x0000000100014a62 a-cfbe649daab6d4af`test::{{impl}}::call_box<(),closure> [inlined] test::run_test::{{closure}} at lib.rs:1480 [opt]
    frame #22: 0x0000000100014a5d a-cfbe649daab6d4af`test::{{impl}}::call_box<(),closure> [inlined] core::ops::function::FnOnce::call_once<closure,(())> at function.rs:223 [opt]
    frame #23: 0x0000000100014a5d a-cfbe649daab6d4af`test::{{impl}}::call_box<(),closure> at lib.rs:141 [opt]
    frame #24: 0x000000010005876f a-cfbe649daab6d4af`panic_unwind::__rust_maybe_catch_panic at lib.rs:99 [opt]
    frame #25: 0x0000000100006ce1 a-cfbe649daab6d4af`std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()> [inlined] std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> at panicking.rs:459 [opt]
    frame #26: 0x0000000100006c9c a-cfbe649daab6d4af`std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()> [inlined] std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> at panic.rs:361 [opt]
    frame #27: 0x0000000100006c9c a-cfbe649daab6d4af`std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()> [inlined] test::run_test::run_test_inner::{{closure}} at lib.rs:1419 [opt]
    frame #28: 0x0000000100006b49 a-cfbe649daab6d4af`std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()> at backtrace.rs:134 [opt]
    frame #29: 0x000000010000f508 a-cfbe649daab6d4af`std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()> [inlined] std::thread::{{impl}}::spawn::{{closure}}::{{closure}}<closure,()> at mod.rs:402 [opt]
    frame #30: 0x000000010000f4f2 a-cfbe649daab6d4af`std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()> [inlined] std::panic::{{impl}}::call_once<(),closure> at panic.rs:296 [opt]
    frame #31: 0x000000010000f4f2 a-cfbe649daab6d4af`std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()> at panicking.rs:480 [opt]
    frame #32: 0x000000010005876f a-cfbe649daab6d4af`panic_unwind::__rust_maybe_catch_panic at lib.rs:99 [opt]
    frame #33: 0x00000001000209c2 a-cfbe649daab6d4af`alloc::boxed::{{impl}}::call_box<(),closure> [inlined] std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> at panicking.rs:459 [opt]
    frame #34: 0x0000000100020989 a-cfbe649daab6d4af`alloc::boxed::{{impl}}::call_box<(),closure> [inlined] std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> at panic.rs:361 [opt]
    frame #35: 0x0000000100020989 a-cfbe649daab6d4af`alloc::boxed::{{impl}}::call_box<(),closure> [inlined] std::thread::{{impl}}::spawn::{{closure}}<closure,()> at mod.rs:401 [opt]
    frame #36: 0x000000010002093c a-cfbe649daab6d4af`alloc::boxed::{{impl}}::call_box<(),closure> at boxed.rs:762 [opt]
    frame #37: 0x000000010004c83c a-cfbe649daab6d4af`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] alloc::boxed::{{impl}}::call_once<(),()> at boxed.rs:772 [opt]
    frame #38: 0x000000010004c839 a-cfbe649daab6d4af`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] std::sys_common::thread::start_thread at thread.rs:24 [opt]
    frame #39: 0x000000010004c7be a-cfbe649daab6d4af`std::sys::imp::thread::{{impl}}::new::thread_start at thread.rs:90 [opt]
    frame #40: 0x00007fff6d23e6c1 libsystem_pthread.dylib`_pthread_body + 340
    frame #41: 0x00007fff6d23e56d libsystem_pthread.dylib`_pthread_start + 377
    frame #42: 0x00007fff6d23dc5d libsystem_pthread.dylib`thread_start + 13
