 sh
> valgrind ./foo
==16282== Memcheck, a memory error detector
==16282== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==16282== Using Valgrind-3.10.1 and LibVEX; rerun with -h for copyright info
==16282== Command: ./foo
==16282== 
==16282== Invalid read of size 1
==16282==    at 0x11B7B1: je_mallocx (in /home/andrew/foo)
==16282==    by 0x10C78F: heap::allocate::hef78f640aa355f8bPda (in /home/andrew/foo)
==16282==    by 0x10C710: main::h2fbc6b3b4aacc11afaa (in /home/andrew/foo)
==16282==    by 0x112D94: sys_common::unwind::try::try_fn::h14601444602367533931 (in /home/andrew/foo)
==16282==    by 0x10FDA8: __rust_try (in /home/andrew/foo)
==16282==    by 0x112A08: rt::lang_start::h7135051af8990a95Zkx (in /home/andrew/foo)
==16282==    by 0x10C7C9: main (in /home/andrew/foo)
==16282==  Address 0x200000000013dd7f is not stack'd, malloc'd or (recently) free'd
==16282== 
==16282== 
==16282== Process terminating with default action of signal 11 (SIGSEGV)
==16282==  General Protection Fault
==16282==    at 0x11B7B1: je_mallocx (in /home/andrew/foo)
==16282==    by 0x10C78F: heap::allocate::hef78f640aa355f8bPda (in /home/andrew/foo)
==16282==    by 0x10C710: main::h2fbc6b3b4aacc11afaa (in /home/andrew/foo)
==16282==    by 0x112D94: sys_common::unwind::try::try_fn::h14601444602367533931 (in /home/andrew/foo)
==16282==    by 0x10FDA8: __rust_try (in /home/andrew/foo)
==16282==    by 0x112A08: rt::lang_start::h7135051af8990a95Zkx (in /home/andrew/foo)
==16282==    by 0x10C7C9: main (in /home/andrew/foo)
==16282== 
==16282== HEAP SUMMARY:
==16282==     in use at exit: 32 bytes in 1 blocks
==16282==   total heap usage: 5 allocs, 4 frees, 976 bytes allocated
==16282== 
==16282== LEAK SUMMARY:
==16282==    definitely lost: 0 bytes in 0 blocks
==16282==    indirectly lost: 0 bytes in 0 blocks
==16282==      possibly lost: 0 bytes in 0 blocks
==16282==    still reachable: 32 bytes in 1 blocks
==16282==         suppressed: 0 bytes in 0 blocks
==16282== Rerun with --leak-check=full to see details of leaked memory
==16282== 
==16282== For counts of detected and suppressed errors, rerun with: -v
==16282== ERROR SUMMARY: 2 errors from 1 contexts (suppressed: 0 from 0)
Segmentation fault (core dumped)
