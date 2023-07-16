
    #0  0x00005555559a157c in arena_dalloc_bin_locked_impl (arena=arena@entry=0x7ffff6712280, chunk=0x7ffff5e00000, ptr=0x7ffff5e0d008, junked=true, bitselm=<optimized out>)
        at /checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c:327
    #1  0x00005555559a2ec5 in je_arena_dalloc_bin_junked_locked (arena=arena@entry=0x7ffff6712280, chunk=<optimized out>, ptr=<optimized out>, bitselm=<optimized out>)
        at /checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c:2746
    #2  0x00005555559b444b in je_tcache_bin_flush_small (tsd=tsd@entry=0x7ffff5dff6a0, tcache=tcache@entry=0x7ffff64af000, tbin=tbin@entry=0x7ffff64af028, binind=binind@entry=0,
        rem=rem@entry=0) at /checkout/src/liballoc_jemalloc/../jemalloc/src/tcache.c:132
    #3  0x00005555559b4e53 in tcache_destroy (tsd=0x7ffff5dff6a0, tcache=0x7ffff64af000) at /checkout/src/liballoc_jemalloc/../jemalloc/src/tcache.c:364
    #4  0x00005555559b5052 in je_tcache_cleanup (tsd=0x7ffff5dff6a0) at /checkout/src/liballoc_jemalloc/../jemalloc/src/tcache.c:403
    #5  0x00005555559b5665 in je_tsd_cleanup (arg=0x7ffff5dff6a0) at /checkout/src/liballoc_jemalloc/../jemalloc/src/tsd.c:82
    #6  0x00007ffff7106439 in __nptl_deallocate_tsd.part.4 () from /lib/x86_64-linux-gnu/libpthread.so.0
    #7  0x00007ffff7107878 in start_thread () from /lib/x86_64-linux-gnu/libpthread.so.0
    #8  0x00007ffff6c2acaf in clone () from /lib/x86_64-linux-gnu/libc.so.6
    