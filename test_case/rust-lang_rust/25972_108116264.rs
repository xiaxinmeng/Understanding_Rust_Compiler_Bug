
Program received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7ffff6ef5700 (LWP 3433)]
0x0000555555580e68 in je_arena_dalloc_bin_locked ()
(gdb) 
(gdb) where
#0  0x0000555555580e68 in je_arena_dalloc_bin_locked ()
#1  0x0000555555593d26 in tcache_destroy ()
#2  0x00005555555945f2 in je_tcache_cleanup ()
#3  0x0000555555596ffd in je_tsd_cleanup ()
#4  0x00007ffff79c03d9 in __nptl_deallocate_tsd.part.5 () from /usr/lib/libpthread.so.0
#5  0x00007ffff79c141d in start_thread () from /usr/lib/libpthread.so.0
#6  0x00007ffff72e1bfd in clone () from /usr/lib/libc.so.6
