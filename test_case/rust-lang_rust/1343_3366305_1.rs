
command: /usr/bin/valgrind --leak-check=full --error-exitcode=100 --quiet --suppressions=../src/etc/x86.supp x86_64-unknown-linux-gnu/test/run-fail/unwind-misc-1.stage1-i686-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
==16269== Thread 4:
==16269== Invalid read of size 1
==16269==    at 0x4A62FFC: _GLOBAL_OFFSET_TABLE_ (in /home/brian/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/i686-unknown-linux-gnu/lib/libstd-79ca5fac56b63fde-0.1.so)
==16269==  Address 0xdc is not stack'd, malloc'd or (recently) free'd
==16269==
==16269==
==16269== Process terminating with default action of signal 11 (SIGSEGV)
==16269==  Access not within mapped region at address 0xDC
==16269==    at 0x4A62FFC: _GLOBAL_OFFSET_TABLE_ (in /home/brian/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/i686-unknown-linux-gnu/lib/libstd-79ca5fac56b63fde-0.1.so)
==16269==  If you believe this happened as a result of a stack
==16269==  overflow in your program's main thread (unlikely but
==16269==  possible), you can try to increase the size of the
==16269==  main thread stack using the --main-stacksize= flag.
==16269==  The main thread stack size used in this run was 16777216.
==16269== Thread 1:
==16269== 64 bytes in 3 blocks are possibly lost in loss record 14 of 35
==16269==    at 0x48DDBD3: malloc (vg_replace_malloc.c:236)
==16269==    by 0x4B15DD0: rust_srv::malloc(unsigned int) (rust_srv.cpp:18)
