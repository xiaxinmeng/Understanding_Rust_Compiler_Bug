
(base) ubuntu@ip-172-31-29-98:~$ gdb rustc
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from rustc...done.
(gdb) run
Starting program: /home/ubuntu/.cargo/bin/rustc 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x0000000000000000 in ?? ()
(gdb) bt
#0  0x0000000000000000 in ?? ()
#1  0x00005555555bd097 in je_malloc_mutex_lock (tsdn=0x0, mutex=0x555555ed56c0 <init_lock>)
    at /checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/mutex.h:101
#2  malloc_init_hard () at /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c:1486
#3  0x0000555555a61795 in malloc_init () at /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c:317
#4  ialloc_body (slow_path=true, usize=<synthetic pointer>, tsdn=<synthetic pointer>, zero=true, size=32)
    at /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c:1583
#5  calloc (num=num@entry=1, size=size@entry=32) at /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c:1824
#6  0x00007ffff6b80627 in _dlerror_run (operate=operate@entry=0x7ffff6b80020 <dlsym_doit>, args=args@entry=0x7fffffffe230) at dlerror.c:141
#7  0x00007ffff6b80088 in __dlsym (handle=<optimized out>, name=<optimized out>) at dlsym.c:70
#8  0x00007ffff6fbd70c in ?? () from /usr/lib/llvm-5.0/lib/clang/5.0.0/lib/linux/libclang_rt.asan-x86_64.so
#9  0x00007ffff708edd3 in ?? () from /usr/lib/llvm-5.0/lib/clang/5.0.0/lib/linux/libclang_rt.asan-x86_64.so
#10 0x00007ffff70b1372 in ?? () from /usr/lib/llvm-5.0/lib/clang/5.0.0/lib/linux/libclang_rt.asan-x86_64.so
#11 0x00007ffff6ffc064 in __interceptor___cxa_atexit () from /usr/lib/llvm-5.0/lib/clang/5.0.0/lib/linux/libclang_rt.asan-x86_64.so
#12 0x00007ffff5b78e56 in ?? () from /usr/lib/x86_64-linux-gnu/libstdc++.so.6
#13 0x00007ffff7de76ba in call_init (l=<optimized out>, argc=argc@entry=1, argv=argv@entry=0x7fffffffe3b8, env=env@entry=0x7fffffffe3c8) at dl-init.c:72
#14 0x00007ffff7de77cb in call_init (env=0x7fffffffe3c8, argv=0x7fffffffe3b8, argc=1, l=<optimized out>) at dl-init.c:30
#15 _dl_init (main_map=0x7ffff7ffe168, argc=1, argv=0x7fffffffe3b8, env=0x7fffffffe3c8) at dl-init.c:120
#16 0x00007ffff7dd7c6a in _dl_start_user () from /lib64/ld-linux-x86-64.so.2
#17 0x0000000000000001 in ?? ()
#18 0x00007fffffffe61d in ?? ()
#19 0x0000000000000000 in ?? ()
(gdb) 
