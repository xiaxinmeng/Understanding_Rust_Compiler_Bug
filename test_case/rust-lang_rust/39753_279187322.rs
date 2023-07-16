
* thread #2: tid = 0x2107e7, 0x00000001000013b4 inf`core::result::unwrap_failed<std::io::error::Error>(msg=<unavailable>, error=<unavailable>) + 4 at result.rs:836, stop reason = breakpoint 1.1
  * frame #0: 0x00000001000013b4 inf`core::result::unwrap_failed<std::io::error::Error>(msg=<unavailable>, error=<unavailable>) + 4 at result.rs:836 [opt]
    frame #1: 0x0000700007049e30
    frame #2: 0x000000010000a4bb inf`panic_unwind::__rust_maybe_catch_panic + 27 at lib.rs:98 [opt]
    frame #3: 0x0000000100001640 inf`alloc::boxed::{{impl}}::call_box<(),closure> + 40 at panicking.rs:434 [opt]
    frame #4: 0x0000000100001618 inf`alloc::boxed::{{impl}}::call_box<(),closure> [inlined] std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> at panic.rs:351 [opt]
    frame #5: 0x0000000100001618 inf`alloc::boxed::{{impl}}::call_box<(),closure> [inlined] std::thread::{{impl}}::spawn::{{closure}}<closure,()> + 53 at mod.rs:301 [opt]
    frame #6: 0x00000001000015e3 inf`alloc::boxed::{{impl}}::call_box<(),closure>(self=0x0000000100620000, args=<unavailable>) + 35 at boxed.rs:605 [opt]
    frame #7: 0x0000000100007bc5 inf`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] alloc::boxed::{{impl}}::call_once<(),()> + 37 at boxed.rs:615 [opt]
    frame #8: 0x0000000100007bbf inf`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] std::sys_common::thread::start_thread + 15 at thread.rs:21 [opt]
    frame #9: 0x0000000100007bb0 inf`std::sys::imp::thread::{{impl}}::new::thread_start + 16 at thread.rs:84 [opt]
    frame #10: 0x00007fffadccbaab libsystem_pthread.dylib`_pthread_body + 180
    frame #11: 0x00007fffadccb9f7 libsystem_pthread.dylib`_pthread_start + 286
    frame #12: 0x00007fffadccb1fd libsystem_pthread.dylib`thread_start + 13
