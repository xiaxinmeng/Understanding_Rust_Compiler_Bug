
$ cargo build && valgrind target/debug/dylib_crash
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
==27114== Memcheck, a memory error detector
==27114== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==27114== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==27114== Command: target/debug/dylib_crash
==27114== 
Dropping lib
Lib is dropped
Success: No thread
Dropping lib
Lib is dropped
==27114== Thread 2:
==27114== Jump to the invalid address stated on the next line
==27114==    at 0x6EC23D0: ???
==27114==    by 0x524B1B7: __nptl_deallocate_tsd.part.5 (in /usr/lib/libpthread-2.26.so)
==27114==    by 0x524C1DC: start_thread (in /usr/lib/libpthread-2.26.so)
==27114==    by 0x577042E: clone (in /usr/lib/libc-2.26.so)
==27114==  Address 0x6ec23d0 is not stack'd, malloc'd or (recently) free'd
==27114== 
==27114== Can't extend stack to 0x402a138 during signal delivery for thread 2:
==27114==   no stack segment
==27114== 
==27114== Process terminating with default action of signal 11 (SIGSEGV): dumping core
==27114==  Access not within mapped region at address 0x402A138
==27114==    at 0x6EC23D0: ???
==27114==    by 0x524B1B7: __nptl_deallocate_tsd.part.5 (in /usr/lib/libpthread-2.26.so)
==27114==    by 0x524C1DC: start_thread (in /usr/lib/libpthread-2.26.so)
==27114==    by 0x577042E: clone (in /usr/lib/libc-2.26.so)
==27114==  If you believe this happened as a result of a stack
==27114==  overflow in your program's main thread (unlikely but
==27114==  possible), you can try to increase the size of the
==27114==  main thread stack using the --main-stacksize= flag.
==27114==  The main thread stack size used in this run was 8388608.
==27114== Invalid write of size 8
==27114==    at 0x4A27630: _vgnU_freeres (in /usr/lib/valgrind/vgpreload_core-amd64-linux.so)
==27114==  Address 0x402aff8 is on thread 2's stack
==27114== 
==27114== 
==27114== Process terminating with default action of signal 11 (SIGSEGV)
==27114==  Access not within mapped region at address 0x402AFF8
==27114==    at 0x4A27630: _vgnU_freeres (in /usr/lib/valgrind/vgpreload_core-amd64-linux.so)
==27114==  If you believe this happened as a result of a stack
==27114==  overflow in your program's main thread (unlikely but
==27114==  possible), you can try to increase the size of the
==27114==  main thread stack using the --main-stacksize= flag.
==27114== 
==27114== HEAP SUMMARY:
==27114==     in use at exit: 384 bytes in 4 blocks
==27114==   total heap usage: 33 allocs, 29 frees, 9,316 bytes allocated
==27114== 
==27114== LEAK SUMMARY:
==27114==    definitely lost: 0 bytes in 0 blocks
==27114==    indirectly lost: 0 bytes in 0 blocks
==27114==      possibly lost: 288 bytes in 1 blocks
==27114==    still reachable: 96 bytes in 3 blocks
==27114==         suppressed: 0 bytes in 0 blocks
==27114== Rerun with --leak-check=full to see details of leaked memory
==27114== 
==27114== For counts of detected and suppressed errors, rerun with: -v
==27114== ERROR SUMMARY: 2 errors from 2 contexts (suppressed: 0 from 0)
[1]    27114 segmentation fault (core dumped)  valgrind target/debug/dylib_crash
