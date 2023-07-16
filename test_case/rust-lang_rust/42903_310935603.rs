
arena_run_tree_remove(rbtree=0x0000000101513340, node=0x0000000102001008) + 186 at arena.c:76, name = 'termdict::tests::test_stream_range_boundaries', stop reason = EXC_BAD_ACCESS (code=1, address=0x8000000)
  * frame #0: 0x00000001007a5a7a tantivy-d77d51b41e725268`arena_run_tree_remove(rbtree=0x0000000101513340, node=0x0000000102001008) + 186 at arena.c:76
    frame #1: 0x00000001007a6215 tantivy-d77d51b41e725268`arena_run_dalloc + 335 at arena.c:1857
    frame #2: 0x00000001007a60c6 tantivy-d77d51b41e725268`arena_run_dalloc(arena=0x0000000101511d80, run=<unavailable>, dirty=true, cleaned=<unavailable>, decommitted=<unavailable>) + 566 at arena.c:1945
    frame #3: 0x00000001007a1574 tantivy-d77d51b41e725268`arena_dalloc_bin_locked_impl [inlined] arena_dalloc_bin_run + 41 at arena.c:2679
    frame #4: 0x00000001007a154b tantivy-d77d51b41e725268`arena_dalloc_bin_locked_impl(arena=<unavailable>, chunk=<unavailable>, ptr=<unavailable>, bitselm=<unavailable>, junked=<unavailable>) + 491 at arena.c:2731
    frame #5: 0x00000001007b4c91 tantivy-d77d51b41e725268`je_tcache_bin_flush_small(tsd=0x000000010168a108, tcache=<unavailable>, tbin=0x00000001016c5048, binind=1, rem=0) + 369 at tcache.c:132
    frame #6: 0x00000001007b5a92 tantivy-d77d51b41e725268`tcache_destroy(tsd=0x000000010168a108, tcache=0x00000001016c5000) + 98 at tcache.c:364
    frame #7: 0x00000001007b5a1a tantivy-d77d51b41e725268`je_tcache_cleanup(tsd=0x0000000101511d80) + 26 at tcache.c:403
    frame #8: 0x00000001007b5fd0 tantivy-d77d51b41e725268`je_tsd_cleanup(arg=0x000000010168a108) + 48 at tsd.c:82
    frame #9: 0x00000001007b6b7a tantivy-d77d51b41e725268`je_tsd_cleanup_wrapper(arg=0x000000010168a100) + 26 at tsd.h:609
    frame #10: 0x00007fffb6e1c4c5 libsystem_pthread.dylib`_pthread_tsd_cleanup + 470
    frame #11: 0x00007fffb6e1c249 libsystem_pthread.dylib`_pthread_exit + 152
    frame #12: 0x00007fffb6e1aab6 libsystem_pthread.dylib`_pthread_body + 191
    frame #13: 0x00007fffb6e1a9f7 libsystem_pthread.dylib`_pthread_start + 286
    frame #14: 0x00007fffb6e1a221 libsystem_pthread.dylib`thread_start + 13

