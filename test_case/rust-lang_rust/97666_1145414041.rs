
root@ede627ca950e:~/sync# valgrind target/debug/sync
==6326== Memcheck, a memory error detector
==6326== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==6326== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==6326== Command: target/debug/sync
==6326== 
==6326== 
==6326== HEAP SUMMARY:
==6326==     in use at exit: 10 bytes in 1 blocks
==6326==   total heap usage: 80 allocs, 79 frees, 36,635 bytes allocated
==6326== 
==6326== LEAK SUMMARY:
==6326==    definitely lost: 0 bytes in 0 blocks
==6326==    indirectly lost: 0 bytes in 0 blocks
==6326==      possibly lost: 0 bytes in 0 blocks
==6326==    still reachable: 10 bytes in 1 blocks
==6326==         suppressed: 0 bytes in 0 blocks
==6326== Rerun with --leak-check=full to see details of leaked memory
==6326== 
==6326== For lists of detected and suppressed errors, rerun with: -s
==6326== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
