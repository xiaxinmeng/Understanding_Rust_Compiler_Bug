
(gdb) backtrace
#0  0x77b3e12c in ?? ()
#1  0x77b3e04f in ?? ()
#2  0x77b3e078 in ?? ()
#3  0x004c7d88 in je_malloc_mutex_lock ()
    at C:/bot/slave/stable-dist-rustc-win-gnu-     
    32/build/src/jemalloc/include/jemalloc/internal/mutex.h:73
#4  malloc_init_hard ()
    at C:/bot/slave/stable-dist-rustc-win-gnu-32/build/src/jemalloc/src/jemalloc.c:692
#5  malloc_init ()
    at C:/bot/slave/stable-dist-rustc-win-gnu-32/build/src/jemalloc/src/jemalloc.c:314
#6  jemalloc_constructor ()
    at C:/bot/slave/stable-dist-rustc-win-gnu-32/build/src/jemalloc/src/jemalloc.c:1994
#7  0x00486eca in __do_global_ctors ()
#8  0x004013b2 in __tmainCRTStartup ()
#9  0x757c495d in KERNEL32!BaseThreadInitThunk ()
    from C:\Windows\SysWOW64\kernel32.dll
#10 0x77b698ee in ?? ()
#11 0x77b698c4 in ?? ()
#12 0x00000000 in ?? ()
