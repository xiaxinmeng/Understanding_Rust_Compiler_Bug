
[futex:rhex/rhex] (master!)% valgrind ./target/debug/rhex
==11605== Memcheck, a memory error detector
==11605== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==11605== Using Valgrind-3.10.1 and LibVEX; rerun with -h for copyright info
==11605== Command: ./target/debug/rhex
==11605== 
==11605== 
==11605== HEAP SUMMARY:
==11605==     in use at exit: 282,430 bytes in 260 blocks
==11605==   total heap usage: 1,289 allocs, 1,029 frees, 529,137 bytes allocated
==11605== 
==11605== LEAK SUMMARY:
==11605==    definitely lost: 0 bytes in 0 blocks
==11605==    indirectly lost: 0 bytes in 0 blocks
==11605==      possibly lost: 1,728 bytes in 3 blocks
==11605==    still reachable: 280,702 bytes in 257 blocks
==11605==         suppressed: 0 bytes in 0 blocks
==11605== Rerun with --leak-check=full to see details of leaked memory
==11605== 
==11605== For counts of detected and suppressed errors, rerun with: -v
==11605== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
