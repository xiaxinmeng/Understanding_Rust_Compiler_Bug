
warning: could not load any Objective-C class information. This will significantly reduce the quality of type information available.
* thread #1: tid = 0x78d98d, 0x00007fff9932dca9 libsystem_platform.dylib`OSSpinLockLock + 7, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x00007fff9932dca9 libsystem_platform.dylib`OSSpinLockLock + 7
    frame #1: 0x00000001000250c6 main`je_arena_dalloc_large [inlined] je_malloc_mutex_lock + 38 at mutex.h:99 [opt]
    frame #2: 0x00000001000250ba main`je_arena_dalloc_large(tsdn=0x000000010060d008, arena=0x4d746c7561666544, chunk=0x0000000100200000, ptr=0x00000001003002c0) + 26 at arena.c:3075 [opt]
    frame #3: 0x0000000100026625 main`je_arena_ralloc [inlined] je_arena_sdalloc(slow_path=true) + 12 at arena.h:1516 [opt]
    frame #4: 0x0000000100026619 main`je_arena_ralloc [inlined] je_isdalloct(slow_path=true) + 164 at jemalloc_internal.h:1195 [opt]
    frame #5: 0x0000000100026575 main`je_arena_ralloc [inlined] je_isqalloc(slow_path=true) at jemalloc_internal.h:1205 [opt]
    frame #6: 0x0000000100026575 main`je_arena_ralloc(tsd=0x000000010060d008, arena=0x0000000000000000, ptr=<unavailable>, oldsize=<unavailable>, size=<unavailable>, alignment=<unavailable>, zero=<unavailable>, tcache=<unavailable>) + 2037 at arena.c:3376 [opt]
    frame #7: 0x000000010001cc79 main`je_rallocx [inlined] je_iralloct(ptr=<unavailable>, oldsize=<unavailable>, alignment=0, tcache=<unavailable>, arena=0x0000000000000000) + 263 at jemalloc_internal.h:1259 [opt]
    frame #8: 0x000000010001cb72 main`je_rallocx(ptr=0x00000001003002c0, size=33, flags=<unavailable>) + 674 at jemalloc.c:2414 [opt]
    frame #9: 0x0000000100019ee1 main`alloc_jemalloc::contents::__rde_realloc + 81 at lib.rs:170 [opt]
    frame #10: 0x0000000100002d68 main`alloc::vec::{{impl}}::reserve_exact<u8> [inlined] alloc::heap::{{impl}}::realloc + 19 at heap.rs:127 [opt]
    frame #11: 0x0000000100002d55 main`alloc::vec::{{impl}}::reserve_exact<u8> [inlined] alloc::raw_vec::{{impl}}::reserve_exact<u8,alloc::heap::Heap> + 28 at raw_vec.rs:429 [opt]
    frame #12: 0x0000000100002d39 main`alloc::vec::{{impl}}::reserve_exact<u8> + 25 at vec.rs:486 [opt]
    frame #13: 0x0000000100006bee main`std::ffi::c_str::{{impl}}::from_vec_unchecked + 30 at c_str.rs:360 [opt]
    frame #14: 0x0000000100006ba2 main`std::ffi::c_str::{{impl}}::_new + 114 at c_str.rs:335 [opt]
    frame #15: 0x00000001000020fc main`std::ffi::c_str::{{impl}}::new<&str>(t=(data_ptr = "../app/target/debug/libapp.dylibget_message", length = 32)) + 60 at c_str.rs:329
    frame #16: 0x0000000100002246 main`main::main + 102 at main.rs:19
    frame #17: 0x000000010003fc0f main`panic_unwind::__rust_maybe_catch_panic + 31 at lib.rs:101 [opt]
    frame #18: 0x000000010000fab9 main`std::rt::lang_start [inlined] std::panicking::try<(),closure> + 51 at panicking.rs:459 [opt]
    frame #19: 0x000000010000fa86 main`std::rt::lang_start [inlined] std::panic::catch_unwind<closure,()> at panic.rs:365 [opt]
    frame #20: 0x000000010000fa86 main`std::rt::lang_start + 422 at rt.rs:58 [opt]
    frame #21: 0x0000000100002705 main`main + 37
    frame #22: 0x00007fff9911f235 libdyld.dylib`start + 1
    frame #23: 0x00007fff9911f235 libdyld.dylib`start + 1
