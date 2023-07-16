
==31040== Memcheck, a memory error detector
==31040== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==31040== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==31040== Command: ./learn
==31040==
==31040== Conditional jump or move depends on uninitialised value(s)
==31040==    at 0x10EB61: std::io::stdio::stdout::stdout_init::hde9bb20c74d239a0 (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x11DC3E: _$LT$std..io..lazy..Lazy$LT$T$GT$$GT$::get::hd42305ba5388eaca (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x10F02E: std::io::stdio::_print::h909f88694efb8b49 (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x10E6EF: learn::main::h86b1088ba10af1af (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x113922: std::panicking::try::do_call::h47aca5fe0bc0ca6f (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x124260: __rust_maybe_catch_panic (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x1138D7: std::panicking::try::haf2827ba761d86f4 (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x110956: std::rt::lang_start::h8c02412f0e1905c4 (in /home/tschottdorf/rust-pthread/learn)
==31040==    by 0x10E729: main (in /home/tschottdorf/rust-pthread/learn)
==31040==  Uninitialised value was created by a stack allocation
==31040==    at 0x10EB02: std::io::stdio::stdout::stdout_init::hde9bb20c74d239a0 (in /home/tschottdorf/rust-pthread/learn)
==31040==
How are you today
==31040==
==31040== HEAP SUMMARY:
==31040==     in use at exit: 0 bytes in 0 blocks
==31040==   total heap usage: 21 allocs, 21 frees, 3,680 bytes allocated
==31040==
==31040== All heap blocks were freed -- no leaks are possible
==31040==
==31040== For counts of detected and suppressed errors, rerun with: -v
==31040== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 0 from 0)
