
$ valgrind ./foo 
==21255== Memcheck, a memory error detector
==21255== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==21255== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==21255== Command: ./foo
==21255== 
Hello World!
How are you today
==21255== 
==21255== HEAP SUMMARY:
==21255==     in use at exit: 312 bytes in 7 blocks
==21255==   total heap usage: 26 allocs, 19 frees, 2,720 bytes allocated
==21255== 
==21255== LEAK SUMMARY:
==21255==    definitely lost: 0 bytes in 0 blocks
==21255==    indirectly lost: 0 bytes in 0 blocks
==21255==      possibly lost: 0 bytes in 0 blocks
==21255==    still reachable: 312 bytes in 7 blocks
==21255==         suppressed: 0 bytes in 0 blocks
==21255== Rerun with --leak-check=full to see details of leaked memory
==21255== 
==21255== For counts of detected and suppressed errors, rerun with: -v
==21255== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
