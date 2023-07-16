
$ valgrind ./exit-flushes
==15994== Memcheck, a memory error detector
==15994== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==15994== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==15994== Command: ./exit-flushes
==15994== 
==15994== 
==15994== HEAP SUMMARY:
==15994==     in use at exit: 32 bytes in 1 blocks
==15994==   total heap usage: 52 allocs, 51 frees, 3,200 bytes allocated
==15994== 
==15994== LEAK SUMMARY:
==15994==    definitely lost: 0 bytes in 0 blocks
==15994==    indirectly lost: 0 bytes in 0 blocks
==15994==      possibly lost: 0 bytes in 0 blocks
==15994==    still reachable: 32 bytes in 1 blocks
==15994==         suppressed: 0 bytes in 0 blocks
==15994== Rerun with --leak-check=full to see details of leaked memory
==15994== 
==15994== For counts of detected and suppressed errors, rerun with: -v
==15994== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
