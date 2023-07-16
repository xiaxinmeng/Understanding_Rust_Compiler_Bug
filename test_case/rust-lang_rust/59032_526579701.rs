
Program received signal SIGSEGV, Segmentation fault.
0x0000000000000000 in ?? ()
(gdb) bt
#0  0x0000000000000000 in ?? ()
#1  0x00007ffff7e3f2d9 in ?? () from /usr/lib64/libesets_pac.so
#2  0x00007ffff7e3f3fd in fcntl () from /usr/lib64/libesets_pac.so
#3  0x00005555555856f6 in os_overcommits_proc () at ../jemalloc/src/pages.c:447
#4  _rjem_je_pages_boot () at ../jemalloc/src/pages.c:579
#5  0x0000555555561931 in malloc_init_hard_a0_locked () at ../jemalloc/src/jemalloc.c:1291
#6  0x000055555555c248 in malloc_init_hard () at ../jemalloc/src/jemalloc.c:1517
#7  malloc_init () at ../jemalloc/src/jemalloc.c:217
#8  imalloc (sopts=<optimized out>, dopts=<optimized out>) at ../jemalloc/src/jemalloc.c:1986
#9  calloc (num=num@entry=1, size=size@entry=32) at ../jemalloc/src/jemalloc.c:2138
#10 0x00007ffff77e7bc9 in _dlerror_run (operate=operate@entry=0x7ffff77e7450 <dlsym_doit>, args=args@entry=0x7fffffffd8d0) at dlerror.c:141
#11 0x00007ffff77e74ff in __dlsym (handle=<optimized out>, name=<optimized out>) at dlsym.c:70
#12 0x00007ffff7e3c69c in ?? () from /usr/lib64/libesets_pac.so
#13 0x00007ffff7e3d91c in ?? () from /usr/lib64/libesets_pac.so
#14 0x00007ffff7e68dc6 in ?? () from /usr/lib64/libesets_pac.so
#15 0x0000000000000001 in ?? ()
#16 0x00007ffff7f8f000 in ?? ()
#17 0x0000000000000001 in ?? ()
#18 0x00007ffff7e3b89b in ?? () from /usr/lib64/libesets_pac.so
#19 0x0000000000000001 in ?? ()
#20 0x00007ffff7fe327a in call_init (l=0x7ffff7f79000, argc=-9856, argc@entry=1, argv=argv@entry=0x7fffffffda18, env=env@entry=0x7fffffffda28) at dl-init.c:58
#21 0x00007ffff7fe33b4 in call_init (env=0x7fffffffda28, argv=0x7fffffffda18, argc=1, l=<optimized out>) at dl-init.c:30
#22 _dl_init (main_map=0x7ffff7ffe130, argc=1, argv=0x7fffffffda18, env=0x7fffffffda28) at dl-init.c:119
#23 0x00007ffff7fd50ca in _dl_start_user () from /lib64/ld-linux-x86-64.so.2
#24 0x0000000000000001 in ?? ()
#25 0x00007fffffffddc6 in ?? ()
#26 0x0000000000000000 in ?? ()
(gdb) quit
