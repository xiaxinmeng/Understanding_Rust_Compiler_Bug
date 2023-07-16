
me@aruba:~/atest$ rustc --version
rustc 1.7.0 (a5d1e7a59 2016-02-29)
me@aruba:~/proggies/atest$ cargo build
Compiling atest v0.1.0 (file:///home/hans/proggies/atest)
me@aruba:~/atest$ valgrind  target/debug/atest
==5975== Memcheck, a memory error detector
==5975== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==5975== Using Valgrind-3.10.0 and LibVEX; rerun with -h for copyright info
==5975== Command: target/debug/atest
==5975== 
Allocated at 0x5e00570
==5975== 
==5975== HEAP SUMMARY:
==5975==     in use at exit: 100 bytes in 1 blocks
==5975==   total heap usage: 7 allocs, 6 frees, 1,124 bytes allocated
==5975== 
==5975== LEAK SUMMARY:
==5975==    definitely lost: 100 bytes in 1 blocks
==5975==    indirectly lost: 0 bytes in 0 blocks
==5975==      possibly lost: 0 bytes in 0 blocks
==5975==    still reachable: 0 bytes in 0 blocks
==5975==         suppressed: 0 bytes in 0 blocks
==5975== Rerun with --leak-check=full to see details of leaked memory
==5975== 
==5975== For counts of detected and suppressed errors, rerun with: -v
==5975== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
