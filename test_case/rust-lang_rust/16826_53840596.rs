
eddy@eddy-laptop ~/P/rust> rustc src/test/run-pass/dst-dtor-2.rs -o dst-dtor-2 -g
src/test/run-pass/dst-dtor-2.rs:21:5: 21:9 warning: code is never used: `f`, #[warn(dead_code)] on by default
src/test/run-pass/dst-dtor-2.rs:21     f: T
                                       ^~~~
eddy@eddy-laptop ~/P/rust> ./dst-dtor-2 
eddy@eddy-laptop ~/P/rust> valgrind ./dst-dtor-2 
==21563== Memcheck, a memory error detector
==21563== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==21563== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==21563== Command: ./dst-dtor-2
==21563== 
==21563== Invalid free() / delete / delete[] / realloc()
==21563==    at 0x1880E8: je_valgrind_freelike_block (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x1121E8: heap::imp::deallocate::he997a48b3c1ea8ccwfa (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x112164: heap::deallocate::h3255d78e44a8dac82ea (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x1120F9: heap::exchange_free::hf43d197ec176b23aEea (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x111F49: Box$LT$Fat$LT$$x5bFoo$x5d$GT$$GT$::glue_drop.1137::ha3d3461cb44cbb67 (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x111B42: main::ha83750fb0f5236b6Kaa (dst-dtor-2.rs:26)
==21563==    by 0x1490A2: start::closure.8430 (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x16199B: rust_try_inner (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x161985: rust_try (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x15EFC2: unwind::try::h03d8d1d4cb0de0c1f7d (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x15EDCE: task::Task::run::hafab6bcab45e61e5zdd (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x148F19: start::h23cb512b711cdb87ooe (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==  Address 0x6014048 is 0 bytes inside a block of size 8 free'd
==21563==    at 0x1880E8: je_valgrind_freelike_block (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x1121E8: heap::imp::deallocate::he997a48b3c1ea8ccwfa (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x112164: heap::deallocate::h3255d78e44a8dac82ea (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x1120F9: heap::exchange_free::hf43d197ec176b23aEea (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x11205C: _$x5bFoo$x5d::glue_drop.1143::h658b915e24b0db1d (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x111F97: Fat$LT$$x5bFoo$x5d$GT$::glue_drop.1140::h2d3c9ca8436fb1fb (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x111F1E: Box$LT$Fat$LT$$x5bFoo$x5d$GT$$GT$::glue_drop.1137::ha3d3461cb44cbb67 (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x111B42: main::ha83750fb0f5236b6Kaa (dst-dtor-2.rs:26)
==21563==    by 0x1490A2: start::closure.8430 (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x16199B: rust_try_inner (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x161985: rust_try (in /home/eddy/Projects/rust/dst-dtor-2)
==21563==    by 0x15EFC2: unwind::try::h03d8d1d4cb0de0c1f7d (in /home/eddy/Projects/rust/dst-dtor-2)
==21563== 
==21563== 
==21563== HEAP SUMMARY:
==21563==     in use at exit: 0 bytes in 0 blocks
==21563==   total heap usage: 12 allocs, 13 frees, 648 bytes allocated
==21563== 
==21563== All heap blocks were freed -- no leaks are possible
==21563== 
==21563== For counts of detected and suppressed errors, rerun with: -v
==21563== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 2 from 2)

