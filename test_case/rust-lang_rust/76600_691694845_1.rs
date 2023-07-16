
Thread 2 (LWP 8923 of process 8923):
#0  0x00007bedce64556a in _lwp_wait () from /usr/lib/libc.so.12
#1  0x00007bedcee0c754 in pthread_join () from /usr/lib/libpthread.so.1
#2  0x0000000000400e8c in main () at ./w.c:51

Thread 1 (LWP 7395 of process 8923):
#0  0x00007bedce6a88ba in ___lwp_park60 () from /usr/lib/libc.so.12
#1  0x00007bedcee09791 in ?? () from /usr/lib/libpthread.so.1
#2  0x00007bedce6d6e1a in je_malloc_mutex_lock_slow () from /usr/lib/libc.so.12
#3  0x00007bedce70e601 in je_arena_choose_hard () from /usr/lib/libc.so.12
#4  0x00007bedce6b594f in je_tsd_tcache_data_init () from /usr/lib/libc.so.12
#5  0x00007bedce6b5b99 in je_tsd_tcache_enabled_data_init () from /usr/lib/libc.so.12
#6  0x00007bedce6b1c19 in je_tsd_fetch_slow () from /usr/lib/libc.so.12
#7  0x00007bedce70fa9c in calloc () from /usr/lib/libc.so.12
#8  0x00007bedcee0af73 in ?? () from /usr/lib/libpthread.so.1
#9  0x00007bedcee0b066 in pthread_attr_get_np () from /usr/lib/libpthread.so.1
#10 0x00007bedcee0b8eb in pthread_getattr_np () from /usr/lib/libpthread.so.1
#11 0x0000000000400d92 in run (arg=0x0) at ./w.c:33
#12 0x00007bedcee0bee0 in ?? () from /usr/lib/libpthread.so.1
#13 0x00007bedce6924e0 in ?? () from /usr/lib/libc.so.12
#14 0x0000000000200000 in ?? ()
#15 0x0000000000000000 in ?? ()
