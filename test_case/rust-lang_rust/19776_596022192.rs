
$ LD_LIBRARY_PATH=./target/debug/ valgrind ./call_rust
==13954== Memcheck, a memory error detector
==13954== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==13954== Using Valgrind-3.14.0 and LibVEX; rerun with -h for copyright info
==13954== Command: ./call_rust
==13954==
Hello, world!
==13954==
==13954== HEAP SUMMARY:
==13954==     in use at exit: 1,200 bytes in 7 blocks
==13954==   total heap usage: 8 allocs, 1 frees, 1,232 bytes allocated
==13954==
==13954== LEAK SUMMARY:
==13954==    definitely lost: 0 bytes in 0 blocks
==13954==    indirectly lost: 0 bytes in 0 blocks
==13954==      possibly lost: 0 bytes in 0 blocks
==13954==    still reachable: 1,200 bytes in 7 blocks
==13954==         suppressed: 0 bytes in 0 blocks
==13954== Rerun with --leak-check=full to see details of leaked memory
==13954==
==13954== For counts of detected and suppressed errors, rerun with: -v
==13954== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
