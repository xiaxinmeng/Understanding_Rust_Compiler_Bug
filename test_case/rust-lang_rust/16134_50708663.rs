
==10722== Memcheck, a memory error detector
==10722== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==10722== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==10722== Command: ./lib
==10722== 
==10722== Conditional jump or move depends on uninitialised value(s)
==10722==    at 0x4B9842: terminfo::parser::compiled::parse::hebbae3f04aa9f209LAa (in /home/apoelstra/test-project/lib)
==10722==    by 0x4CA77E: terminfo::TerminfoTerminal$LT$T$GT$.Terminal$LT$T$GT$::new::h5799471108736378461 (in /home/apoelstra/test-project/lib)
==10722==    by 0x4CA0D6: stdout::h08598f4cc0d16cb2ahc (in /home/apoelstra/test-project/lib)
==10722==    by 0x442896: run_tests_console::h2805fd0a2fd8a1bcuFc (in /home/apoelstra/test-project/lib)
==10722==    by 0x440A91: test_main::h48522b4de40e1fa6rvb (in /home/apoelstra/test-project/lib)
==10722==    by 0x44B960: test_main_static::h7a67716904b9cb3eczb (in /home/apoelstra/test-project/lib)
==10722==    by 0x407F2A: __test::main::hf2a689af4b08782anaa (in /home/apoelstra/test-project/lib)
==10722==    by 0x4401D2: start::closure.$x22closure$x22$LP$8271$RP$ (in /home/apoelstra/test-project/lib)
==10722==    by 0x507B0B: rust_try (in /home/apoelstra/test-project/lib)
==10722==    by 0x504D96: unwind::try::hbf56caf860a163e6XOd (in /home/apoelstra/test-project/lib)
==10722==    by 0x504B84: task::Task::run::h6627b5a080a9a32fzWc (in /home/apoelstra/test-project/lib)
==10722==    by 0x43FFEC: start::h1f86d2536e8f1a682pe (in /home/apoelstra/test-project/lib)
==10722== 

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

==10722== 
==10722== HEAP SUMMARY:
==10722==     in use at exit: 0 bytes in 0 blocks
==10722==   total heap usage: 534 allocs, 534 frees, 155,576 bytes allocated
==10722== 
==10722== All heap blocks were freed -- no leaks are possible
==10722== 
==10722== For counts of detected and suppressed errors, rerun with: -v
==10722== Use --track-origins=yes to see where uninitialised values come from
==10722== ERROR SUMMARY: 368 errors from 1 contexts (suppressed: 2 from 2)
