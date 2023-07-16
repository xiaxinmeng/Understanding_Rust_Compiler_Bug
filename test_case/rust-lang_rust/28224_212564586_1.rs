
me@aruba:~/atest$ rustc --version
rustc 1.8.0 (db2939409 2016-04-11)
me@aruba:~/atest$ cargo build
me@aruba:~/atest$ valgrind  target/debug/atest
==2921== Memcheck, a memory error detector
==2921== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==2921== Using Valgrind-3.10.0 and LibVEX; rerun with -h for copyright info
==2921== Command: target/debug/atest
==2921== 
Allocated at 0x6035000
==2921== 
==2921== HEAP SUMMARY:
==2921==     in use at exit: 0 bytes in 0 blocks
==2921==   total heap usage: 0 allocs, 0 frees, 0 bytes allocated
==2921== 
==2921== All heap blocks were freed -- no leaks are possible
==2921== 
==2921== For counts of detected and suppressed errors, rerun with: -v
==2921== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
