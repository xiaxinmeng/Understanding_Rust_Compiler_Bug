
$ valgrind --leak-check=full --show-reachable=yes ./learn
==3771== Memcheck, a memory error detector
==3771== Copyright (C) 2002-2012, and GNU GPL'd, by Julian Seward et al.
==3771== Using Valgrind-3.8.1 and LibVEX; rerun with -h for copyright info
==3771== Command: ./learn
==3771== 
Hello World!
How are you today
==3771== 
==3771== HEAP SUMMARY:
==3771==     in use at exit: 1,184 bytes in 4 blocks
==3771==   total heap usage: 18 allocs, 14 frees, 2,696 bytes allocated
==3771== 
==3771== 32 bytes in 1 blocks are still reachable in loss record 1 of 4
==3771==    at 0x128A12: je_mallocx (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11137F: thread_local::imp::register_dtor::h6d14515a39f84cf203b (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x1129E3: thread_local::Key$LT$T$GT$::with::h7241079596358413660 (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x112C42: io::stdio::with_task_stdout::hafb602ec3975fdc5syg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x115557: io::stdio::println_args::h7f0794513d4f2dfcxDg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x1110B7: main::hbd01322dca53aee9eaa (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A22A: rt::start::closure.32122 (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FEB: rust_try_inner (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FD5: rust_try (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123BB2: unwind::try::hf2f7fcf7ecc46c43Tyc (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123A8B: task::Task::run::h911f3b3bbb0c433efKb (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A05B: rt::start::hfd264fa826df3608S9x (in /home/rjammalamadaka/Programs/Rust/learn)
==3771== 
==3771== 64 bytes in 1 blocks are still reachable in loss record 2 of 4
==3771==    at 0x128A12: je_mallocx (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x111431: thread_local::imp::register_dtor::h6d14515a39f84cf203b (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x1129E3: thread_local::Key$LT$T$GT$::with::h7241079596358413660 (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x112C42: io::stdio::with_task_stdout::hafb602ec3975fdc5syg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x115557: io::stdio::println_args::h7f0794513d4f2dfcxDg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x1110B7: main::hbd01322dca53aee9eaa (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A22A: rt::start::closure.32122 (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FEB: rust_try_inner (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FD5: rust_try (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123BB2: unwind::try::hf2f7fcf7ecc46c43Tyc (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123A8B: task::Task::run::h911f3b3bbb0c433efKb (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A05B: rt::start::hfd264fa826df3608S9x (in /home/rjammalamadaka/Programs/Rust/learn)
==3771== 
==3771== 64 bytes in 1 blocks are still reachable in loss record 3 of 4
==3771==    at 0x128A12: je_mallocx (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x112CDD: io::stdio::with_task_stdout::hafb602ec3975fdc5syg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x115557: io::stdio::println_args::h7f0794513d4f2dfcxDg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x1110B7: main::hbd01322dca53aee9eaa (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A22A: rt::start::closure.32122 (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FEB: rust_try_inner (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FD5: rust_try (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123BB2: unwind::try::hf2f7fcf7ecc46c43Tyc (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123A8B: task::Task::run::h911f3b3bbb0c433efKb (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A05B: rt::start::hfd264fa826df3608S9x (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x119E35: rt::lang_start::hde2a214462357c7eb9x (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11117E: main (in /home/rjammalamadaka/Programs/Rust/learn)
==3771== 
==3771== 1,024 bytes in 1 blocks are still reachable in loss record 4 of 4
==3771==    at 0x128A12: je_mallocx (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x112222: io::buffered::BufferedWriter$LT$W$GT$::with_capacity::h8206835914802320726 (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x112D63: io::stdio::with_task_stdout::hafb602ec3975fdc5syg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x115557: io::stdio::println_args::h7f0794513d4f2dfcxDg (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x1110B7: main::hbd01322dca53aee9eaa (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A22A: rt::start::closure.32122 (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FEB: rust_try_inner (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x124FD5: rust_try (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123BB2: unwind::try::hf2f7fcf7ecc46c43Tyc (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x123A8B: task::Task::run::h911f3b3bbb0c433efKb (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x11A05B: rt::start::hfd264fa826df3608S9x (in /home/rjammalamadaka/Programs/Rust/learn)
==3771==    by 0x119E35: rt::lang_start::hde2a214462357c7eb9x (in /home/rjammalamadaka/Programs/Rust/learn)
==3771== 
==3771== LEAK SUMMARY:
==3771==    definitely lost: 0 bytes in 0 blocks
==3771==    indirectly lost: 0 bytes in 0 blocks
==3771==      possibly lost: 0 bytes in 0 blocks
==3771==    still reachable: 1,184 bytes in 4 blocks
==3771==         suppressed: 0 bytes in 0 blocks
==3771== 
==3771== For counts of detected and suppressed errors, rerun with: -v
==3771== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 6 from 6)
