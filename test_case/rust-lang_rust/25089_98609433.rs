
(lldb) run
Process 30437 launched: '/Users/zummenix/projects/play/target/debug/play-bc5fff1cb75e27c9' (x86_64)

running 1 test
Process 30437 stopped
* thread #2: tid = 0x20a9f5, 0x0000000100063bf6 play-bc5fff1cb75e27c9`je_arena_dalloc_bin_locked [inlined] je_bitmap_unset(bit=0) + 71 at bitmap.h:215, name = 'panic', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x0000000100063bf6 play-bc5fff1cb75e27c9`je_arena_dalloc_bin_locked [inlined] je_bitmap_unset(bit=0) + 71 at bitmap.h:215
(lldb) bt
* thread #2: tid = 0x20a9f5, 0x0000000100063bf6 play-bc5fff1cb75e27c9`je_arena_dalloc_bin_locked [inlined] je_bitmap_unset(bit=0) + 71 at bitmap.h:215, name = 'panic', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x0000000100063bf6 play-bc5fff1cb75e27c9`je_arena_dalloc_bin_locked [inlined] je_bitmap_unset(bit=0) + 71 at bitmap.h:215
    frame #1: 0x0000000100063baf play-bc5fff1cb75e27c9`je_arena_dalloc_bin_locked [inlined] arena_run_reg_dalloc(ptr=<unavailable>, run=0x0000000101c021a8) + 95 at arena.c:217
    frame #2: 0x0000000100063b50 play-bc5fff1cb75e27c9`je_arena_dalloc_bin_locked(arena=0x0000000101042980, chunk=0x0000000101c00000, ptr=<unavailable>, bitselm=0x0000000000000000) + 208 at arena.c:1780
    frame #3: 0x0000000100074381 play-bc5fff1cb75e27c9`je_tcache_bin_flush_small(tbin=0x0000000101c1c028, binind=0, rem=0, tcache=0x0000000101c1c000) + 241 at tcache.c:120
    frame #4: 0x0000000100074f50 play-bc5fff1cb75e27c9`je_tcache_cleanup [inlined] tcache_destroy + 42 at tcache.c:328
    frame #5: 0x0000000100074f26 play-bc5fff1cb75e27c9`je_tcache_cleanup(tsd=0x0000000101433108) + 38 at tcache.c:385
    frame #6: 0x00000001000769ff play-bc5fff1cb75e27c9`je_tsd_cleanup(arg=0x0000000101433108) + 159 at tsd.c:81
    frame #7: 0x0000000100076e2a play-bc5fff1cb75e27c9`je_tsd_cleanup_wrapper(arg=0x0000000101433100) + 26 at tsd.h:486
    frame #8: 0x00007fff968bb72a libsystem_pthread.dylib`_pthread_tsd_cleanup + 86
    frame #9: 0x00007fff968bb451 libsystem_pthread.dylib`_pthread_exit + 117
    frame #10: 0x00007fff968ba273 libsystem_pthread.dylib`_pthread_body + 142
    frame #11: 0x00007fff968ba1e5 libsystem_pthread.dylib`_pthread_start + 176
    frame #12: 0x00007fff968b841d libsystem_pthread.dylib`thread_start + 13
