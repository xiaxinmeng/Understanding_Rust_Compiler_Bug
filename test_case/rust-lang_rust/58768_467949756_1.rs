
==28634== Memcheck, a memory error detector
==28634== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==28634== Using Valgrind-3.14.0 and LibVEX; rerun with -h for copyright info
==28634== Command: ./target/debug/test_lazy
==28634== 
==28634== 
==28634== HEAP SUMMARY:
==28634==     in use at exit: 1,144 bytes in 3 blocks
==28634==   total heap usage: 19 allocs, 16 frees, 3,409 bytes allocated
==28634== 
==28634== LEAK SUMMARY:
==28634==    definitely lost: 80 bytes in 1 blocks
==28634==    indirectly lost: 1,064 bytes in 2 blocks
==28634==      possibly lost: 0 bytes in 0 blocks
==28634==    still reachable: 0 bytes in 0 blocks
==28634==         suppressed: 0 bytes in 0 blocks
==28634== Rerun with --leak-check=full to see details of leaked memory
==28634== 
==28634== For counts of detected and suppressed errors, rerun with: -v
==28634== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
