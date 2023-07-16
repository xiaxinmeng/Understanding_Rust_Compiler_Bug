
 malloc: *** error for object 0x100202a30: pointer being freed was not allocated
 thread #1: tid = 0x24c63, 0x00007fffb6d31dda libsystem_kernel.dylib`__pthread_kill + 10, queue = 'com.apple.main-thread', stop reason = signal SIGABRT
  * frame #0: 0x00007fffb6d31dda libsystem_kernel.dylib`__pthread_kill + 10
    frame #1: 0x00007fffb6e1d787 libsystem_pthread.dylib`pthread_kill + 90
    frame #2: 0x00007fffb6c97420 libsystem_c.dylib`abort + 129
    frame #3: 0x00007fffb6d87097 libsystem_malloc.dylib`free + 530
    frame #4: 0x000000010001011f test-tantivy`alloc::raw_vec::{{impl}}::dealloc_buffer<u8,alloc::heap::HeapAlloc>(self=0x00007fff5fbff208) + 223 at raw_vec.rs:637
    frame #5: 0x000000010001cced test-tantivy`alloc::raw_vec::{{impl}}::drop<u8,alloc::heap::HeapAlloc>(self=0x00007fff5fbff208) + 29 at raw_vec.rs:645
    frame #6: 0x0000000100014b65 test-tantivy`core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8, alloc::heap::HeapAlloc>>((null)=0x00007fff5fbff208) + 21 at ptr.rs:60
    frame #7: 0x0000000100014870 test-tantivy`core::ptr::drop_in_place<alloc::vec::Vec<u8>>((null)=0x00007fff5fbff208) + 64 at ptr.rs:60
    frame #8: 0x0000000100013cb9 test-tantivy`core::ptr::drop_in_place<fst::raw::Stream<fst::inner_automaton::AlwaysMatch>>((null)=0x00007fff5fbff200) + 25 at ptr.rs:60
    frame #9: 0x00000001000140a5 test-tantivy`core::ptr::drop_in_place<fst::inner_map::Stream<fst::inner_automaton::AlwaysMatch>>((null)=0x00007fff5fbff200) + 21 at ptr.rs:60
    frame #10: 0x0000000100013789 test-tantivy`core::ptr::drop_in_place<tantivy::termdict::fstdict::streamer::TermStreamerImpl<u32>>((null)=0x00007fff5fbff1f8) + 25 at ptr.rs:60
    frame #11: 0x0000000100020761 test-tantivy`test_tantivy::main + 5857 at main.rs:75
    frame #12: 0x000000010005d88d test-tantivy`panic_unwind::__rust_maybe_catch_panic + 29 at lib.rs:98
    frame #13: 0x000000010005cd85 test-tantivy`std::rt::lang_start [inlined] std::panicking::try<(),closure> + 51 at panicking.rs:433
    frame #14: 0x000000010005cd52 test-tantivy`std::rt::lang_start [inlined] std::panic::catch_unwind<closure,()> at panic.rs:361
    frame #15: 0x000000010005cd52 test-tantivy`std::rt::lang_start + 434 at rt.rs:59
    frame #16: 0x0000000100020fea test-tantivy`main + 42
    frame #17: 0x00007fffb6c03255 libdyld.dylib`start + 1
