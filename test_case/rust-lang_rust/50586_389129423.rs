`
$ sudo valgrind ./target/release/sxi
Password:
==17563== Memcheck, a memory error detector
==17563== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==17563== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==17563== Command: ./target/release/sxi
==17563==
--17563-- run: /usr/bin/dsymutil "./target/release/sxi"
Press Ctrl-D or enter "quit" to exit.
>> "foo"
==17563==
==17563== Process terminating with default action of signal 11 (SIGSEGV)
==17563==  General Protection Fault
==17563==    at 0x1004BDD4B: thread_local::thread_id::get (in ./target/release/sxi)
==17563==    by 0x100471960: regex::re_unicode::Regex::find_at (in ./target/release/sxi)
==17563==    by 0x10046EBB0: regex::re_unicode::Regex::find (in ./target/release/sxi)
==17563==    by 0x100409777: PARSER_CRATE::lexer::Lexer::read_regex (in ./target/release/sxi)
==17563==    by 0x10040A47B: PARSER_CRATE::lex_rules::whitespace (in ./target/release/sxi)
==17563==    by 0x10030CE3E: core::ops::function::Fn::call (in ./target/release/sxi)
                          [redacted]
==17563==    by 0x100407BF0: core::ops::function::Fn::call (in ./target/release/sxi)
==17563==
==17563== HEAP SUMMARY:
==17563==     in use at exit: 42,555 bytes in 432 blocks
==17563==   total heap usage: 520 allocs, 88 frees, 52,289 bytes allocated
==17563==
==17563== LEAK SUMMARY:
==17563==    definitely lost: 0 bytes in 0 blocks
==17563==    indirectly lost: 0 bytes in 0 blocks
==17563==      possibly lost: 0 bytes in 0 blocks
==17563==    still reachable: 7,575 bytes in 15 blocks
==17563==         suppressed: 34,980 bytes in 417 blocks
==17563== Rerun with --leak-check=full to see details of leaked memory
==17563==
==17563== For counts of detected and suppressed errors, rerun with: -v
==17563== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
