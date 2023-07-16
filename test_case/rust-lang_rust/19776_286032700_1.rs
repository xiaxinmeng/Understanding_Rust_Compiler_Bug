
valgrind --track-origins=yes --leak-check=full --show-reachable=yes ./learn
==27188== Memcheck, a memory error detector
==27188== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==27188== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==27188== Command: ./learn
==27188==
==27188== Conditional jump or move depends on uninitialised value(s)
==27188==    at 0x119CB1: std::io::stdio::stdout::stdout_init::hde9bb20c74d239a0 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x117D3E: _$LT$std..io..lazy..Lazy$LT$T$GT$$GT$::get::hd42305ba5388eaca (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11A17E: std::io::stdio::_print::h909f88694efb8b49 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E6FF: learn::main::h86b1088ba10af1af (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11F292: std::panicking::try::do_call::h47aca5fe0bc0ca6f (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x124200: __rust_maybe_catch_panic (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11F247: std::panicking::try::haf2827ba761d86f4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11B498: std::panic::catch_unwind::hd0e9dc562574f6f9 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10F206: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E739: main (in /home/tschottdorf/rust-pthread/learn)
==27188==  Uninitialised value was created by a stack allocation
==27188==    at 0x119C52: std::io::stdio::stdout::stdout_init::hde9bb20c74d239a0 (in /home/tschottdorf/rust-pthread/learn)
==27188==
How are you today
==27188==
==27188== HEAP SUMMARY:
==27188==     in use at exit: 312 bytes in 7 blocks
==27188==   total heap usage: 22 allocs, 15 frees, 3,744 bytes allocated
==27188==
==27188== 8 bytes in 1 blocks are still reachable in loss record 1 of 7
==27188==    at 0x129DED: mallocx (jemalloc.c:2173)
==27188==    by 0x123F30: _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::with_capacity::h9ac4354317bda4cc (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x124062: collections::slice::_$LT$impl$u20$collections..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$::to_owned::ha76a6ec29817e7b5 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x123EE1: collections::str::_$LT$impl$u20$collections..borrow..ToOwned$u20$for$u20$str$GT$::to_owned::h471d5925d3705d5e (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10F1D0: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E739: main (in /home/tschottdorf/rust-pthread/learn)
==27188==
==27188== 32 bytes in 1 blocks are still reachable in loss record 2 of 7
==27188==    at 0x129DED: mallocx (jemalloc.c:2173)
==27188==    by 0x10E906: _$LT$std..thread..local..os..Key$LT$T$GT$$GT$::get::hf79eb5e1178b3ce0 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11141D: _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::haff1cf65b443d42b (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11F2B8: std::panicking::panicking::h4bf7d1f6b8817da7 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x1103BA: _$LT$std..sys_common..remutex..ReentrantMutex$LT$T$GT$$GT$::lock::h1f21b2cbcd0ed8ce (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x119E41: _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::hb0aeab83fe2aee50 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11A1D0: std::io::stdio::_print::h909f88694efb8b49 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E6FF: learn::main::h86b1088ba10af1af (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11F292: std::panicking::try::do_call::h47aca5fe0bc0ca6f (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x124200: __rust_maybe_catch_panic (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11F247: std::panicking::try::haf2827ba761d86f4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11B498: std::panic::catch_unwind::hd0e9dc562574f6f9 (in /home/tschottdorf/rust-pthread/learn)
==27188==
==27188== 48 bytes in 1 blocks are still reachable in loss record 3 of 7
==27188==    at 0x129DED: mallocx (jemalloc.c:2173)
==27188==    by 0x1165E0: _$LT$std..sync..mutex..Mutex$LT$T$GT$$GT$::new::hf4e3619dc58c81a2 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x110856: std::thread::Thread::new::ha2a11efc1d6ac4f2 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x110B5C: _$LT$std..thread..Thread$u20$as$u20$std..sys_common..thread_info..NewThread$GT$::new::h188fe40d655e3e09 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10F1D8: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E739: main (in /home/tschottdorf/rust-pthread/learn)
==27188==
==27188== 48 bytes in 1 blocks are still reachable in loss record 4 of 7
==27188==    at 0x129DED: mallocx (jemalloc.c:2173)
==27188==    by 0x1100F5: std::sync::condvar::Condvar::new::h4fbbad5b416fb56f (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x110860: std::thread::Thread::new::ha2a11efc1d6ac4f2 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x110B5C: _$LT$std..thread..Thread$u20$as$u20$std..sys_common..thread_info..NewThread$GT$::new::h188fe40d655e3e09 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10F1D8: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E739: main (in /home/tschottdorf/rust-pthread/learn)
==27188==
==27188== 48 bytes in 1 blocks are still reachable in loss record 5 of 7
==27188==    at 0x129DED: mallocx (jemalloc.c:2173)
==27188==    by 0x10E876: _$LT$std..thread..local..os..Key$LT$T$GT$$GT$::get::h307be15b67cc5cff (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x1117FB: _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::hb38a09bf838c22e7 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11E5FD: std::sys_common::thread_info::set::h9bdea53d21fd0ea4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10F1EF: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E739: main (in /home/tschottdorf/rust-pthread/learn)
==27188==
==27188== 48 bytes in 1 blocks are still reachable in loss record 6 of 7
==27188==    at 0x129DED: mallocx (jemalloc.c:2173)
==27188==    by 0x10E7E6: _$LT$std..thread..local..os..Key$LT$T$GT$$GT$::get::h0a66dfc1cd407d9b (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x111902: _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::state::h517ffedf5ce521c1 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11A156: std::io::stdio::_print::h909f88694efb8b49 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E6FF: learn::main::h86b1088ba10af1af (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11F292: std::panicking::try::do_call::h47aca5fe0bc0ca6f (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x124200: __rust_maybe_catch_panic (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11F247: std::panicking::try::haf2827ba761d86f4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x11B498: std::panic::catch_unwind::hd0e9dc562574f6f9 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10F206: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E739: main (in /home/tschottdorf/rust-pthread/learn)
==27188==
==27188== 80 bytes in 1 blocks are still reachable in loss record 7 of 7
==27188==    at 0x129DED: mallocx (jemalloc.c:2173)
==27188==    by 0x110896: std::thread::Thread::new::ha2a11efc1d6ac4f2 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x110B5C: _$LT$std..thread..Thread$u20$as$u20$std..sys_common..thread_info..NewThread$GT$::new::h188fe40d655e3e09 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10F1D8: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==27188==    by 0x10E739: main (in /home/tschottdorf/rust-pthread/learn)
==27188==
==27188== LEAK SUMMARY:
==27188==    definitely lost: 0 bytes in 0 blocks
==27188==    indirectly lost: 0 bytes in 0 blocks
==27188==      possibly lost: 0 bytes in 0 blocks
==27188==    still reachable: 312 bytes in 7 blocks
==27188==         suppressed: 0 bytes in 0 blocks
==27188==
==27188== For counts of detected and suppressed errors, rerun with: -v
==27188== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 0 from 0)
