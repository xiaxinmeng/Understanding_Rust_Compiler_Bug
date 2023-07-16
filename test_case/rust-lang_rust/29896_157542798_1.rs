
$ valgrind --show-leak-kinds=all --leak-check=full ./exit-flushes
==16010== Memcheck, a memory error detector
==16010== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==16010== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==16010== Command: ./exit-flushes
==16010== 
==16010== 
==16010== HEAP SUMMARY:
==16010==     in use at exit: 32 bytes in 1 blocks
==16010==   total heap usage: 52 allocs, 51 frees, 3,200 bytes allocated
==16010== 
==16010== 32 bytes in 1 blocks are still reachable in loss record 1 of 1
==16010==    at 0x4C2CC70: calloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==16010==    by 0x4E3868F: _dlerror_run (dlerror.c:141)
==16010==    by 0x4E380C0: dlopen@@GLIBC_2.2.5 (dlopen.c:87)
==16010==    by 0x115427: dynamic_lib::_$LT$impl$GT$::open::h18d9049daa783e54g0d (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x11E675: sys::thread::_$LT$impl$GT$::new::h12092075689a77b6yFw (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x11BB4F: process::_$LT$impl$GT$::wait_with_output::h9de87472d42d007c8Qm (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x11B86C: process::_$LT$impl$GT$::output::hc2dfccaa9143d1c54Cm (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x11034A: main::h82a74a745ccbaed7iaa (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x1256B4: sys_common::unwind::try::try_fn::h5598476508523168281 (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x1229C8: __rust_try (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x125356: rt::lang_start::he15236a7da0e712coox (in /home/alex/code/rust4/exit-flushes)
==16010==    by 0x114329: main (in /home/alex/code/rust4/exit-flushes)
==16010== 
==16010== LEAK SUMMARY:
==16010==    definitely lost: 0 bytes in 0 blocks
==16010==    indirectly lost: 0 bytes in 0 blocks
==16010==      possibly lost: 0 bytes in 0 blocks
==16010==    still reachable: 32 bytes in 1 blocks
==16010==         suppressed: 0 bytes in 0 blocks
==16010== 
==16010== For counts of detected and suppressed errors, rerun with: -v
==16010== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
