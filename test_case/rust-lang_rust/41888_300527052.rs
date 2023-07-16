
$ valgrind ./parse
==1034== Memcheck, a memory error detector
==1034== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==1034== Using Valgrind-3.12.0 and LibVEX; rerun with -h for copyright info
==1034== Command: ./parse
==1034== 
expression_x loop
branch 1
expression_x loop
branch 2
expression_x loop
branch 1
expression_x loop
branch 2
==1034== Invalid read of size 8
==1034==    at 0x10F2C0: core::ptr::drop_in_place::h4d1aff9439cb72f2 (in /home/dwrensha/Desktop/peresil-segfault/parse)
==1034==    by 0x1: ???
==1034==    by 0x1442C3: ??? (in /home/dwrensha/Desktop/peresil-segfault/parse)
==1034==  Address 0x1 is not stack'd, malloc'd or (recently) free'd
==1034== 
==1034== 
==1034== Process terminating with default action of signal 11 (SIGSEGV)
==1034==  Access not within mapped region at address 0x1
==1034==    at 0x10F2C0: core::ptr::drop_in_place::h4d1aff9439cb72f2 (in /home/dwrensha/Desktop/peresil-segfault/parse)
==1034==    by 0x1: ???
==1034==    by 0x1442C3: ??? (in /home/dwrensha/Desktop/peresil-segfault/parse)
==1034==  If you believe this happened as a result of a stack
==1034==  overflow in your program's main thread (unlikely but
==1034==  possible), you can try to increase the size of the
==1034==  main thread stack using the --main-stacksize= flag.
==1034==  The main thread stack size used in this run was 8388608.
==1034== 
==1034== HEAP SUMMARY:
==1034==     in use at exit: 64 bytes in 2 blocks
==1034==   total heap usage: 7 allocs, 5 frees, 2,032 bytes allocated
==1034== 
==1034== LEAK SUMMARY:
==1034==    definitely lost: 0 bytes in 0 blocks
==1034==    indirectly lost: 0 bytes in 0 blocks
==1034==      possibly lost: 0 bytes in 0 blocks
==1034==    still reachable: 64 bytes in 2 blocks
==1034==         suppressed: 0 bytes in 0 blocks
==1034== Rerun with --leak-check=full to see details of leaked memory
==1034== 
==1034== For counts of detected and suppressed errors, rerun with: -v
==1034== ERROR SUMMARY: 2 errors from 1 contexts (suppressed: 0 from 0)
Segmentation fault

