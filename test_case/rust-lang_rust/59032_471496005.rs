(gdb) run
Starting program: /usr/local/bin/rustc --version
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x0000000000000000 in ?? ()
(gdb) bt
#0  0x0000000000000000 in ?? ()
#1  0x00007ffff7e8757c in ?? () from /usr/lib/libesets_pac.so
#2  0x00007ffff7e876a0 in fcntl () from /usr/lib/libesets_pac.so
#3  0x0000555555585656 in os_overcommits_proc ()
    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-539c8a7040e7c5bc/out/jemalloc/src/pages.c:447
#4  _rjem_je_pages_boot ()
    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-539c8a7040e7c5bc/out/jemalloc/src/pages.c:579
#5  0x0000555555561911 in malloc_init_hard_a0_locked ()
    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-539c8a7040e7c5bc/out/jemalloc/src/jemalloc.c:1291
#6  0x000055555555c258 in malloc_init_hard ()
    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-539c8a7040e7c5bc/out/jemalloc/src/jemalloc.c:1517
#7  malloc_init ()
    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-539c8a7040e7c5bc/out/jemalloc/src/jemalloc.c:217
#8  imalloc (sopts=<optimized out>, dopts=<optimized out>)
    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-539c8a7040e7c5bc/out/jemalloc/src/jemalloc.c:1986
#9  calloc (num=1, size=32)
    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-539c8a7040e7c5bc/out/jemalloc/src/jemalloc.c:2138
#10 0x00007ffff726b627 in ?? () from /lib/x86_64-linux-gnu/libdl.so.2
#11 0x00007ffff726b088 in dlsym () from /lib/x86_64-linux-gnu/libdl.so.2
#12 0x00007ffff7e847c9 in ?? () from /usr/lib/libesets_pac.so
#13 0x00007ffff7e85aa7 in ?? () from /usr/lib/libesets_pac.so
---Type <return> to continue, or q <return> to quit---
#14 0x00007ffff7eb1256 in ?? () from /usr/lib/libesets_pac.so
#15 0x0000000000000002 in ?? ()
#16 0x00007ffff7fd7000 in ?? ()
#17 0x0000000000000002 in ?? ()
#18 0x00007ffff7e8389b in ?? () from /usr/lib/libesets_pac.so
#19 0x0000000000000002 in ?? ()
#20 0x00007ffff7de767a in ?? () from /lib64/ld-linux-x86-64.so.2
#21 0x00007ffff7de77cb in ?? () from /lib64/ld-linux-x86-64.so.2
#22 0x00007ffff7dd7c6a in ?? () from /lib64/ld-linux-x86-64.so.2
#23 0x0000000000000002 in ?? ()
#24 0x00007fffffffe028 in ?? ()
#25 0x00007fffffffe03d in ?? ()
#26 0x0000000000000000 in ?? ()
