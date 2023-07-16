
$ valgrind ./foo
==9880== Memcheck, a memory error detector
==9880== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==9880== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==9880== Command: ./foo
==9880== 
==9880== Invalid read of size 8
==9880==    at 0x403AC2: cell::Cell$LT$T$GT$::get::h14749520627650063431::v0.0 (in /home/alex/foo)
==9880==    by 0x403C2A: cell::Ref$LT$$x27b$C$$x20T$GT$.Drop::drop::h5517615880807730945::v0.0 (in /home/alex/foo)
==9880==    by 0x40406A: core..cell..Ref$LT$$x27_$C$int$GT$::glue_drop.1537::h7a74728680c53b3a (in /home/alex/foo)
==9880==    by 0x4031E1: main::hfd2b07b8610388fafaa::v0.0 (in /home/alex/foo)
==9880==    by 0x447512: start::closure.7199 (in /home/alex/foo)
==9880==    by 0x463862: task::Task::run::closure.5303 (in /home/alex/foo)
==9880==    by 0x466C4B: rust_try (in /home/alex/foo)
==9880==    by 0x465565: unwind::try::h7d720262cba71fd0fGd::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x463714: task::Task::run::hac578e5b353eedcbVVc::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x4472CD: start::h0149aeff7c3a4b69Jme::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x447098: lang_start::h3e00b03c17efac073le::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x40332E: main (in /home/alex/foo)
==9880==  Address 0x6014058 is 8 bytes inside a block of size 16 free'd
==9880==    at 0x471379: je_dallocx (in /home/alex/foo)
==9880==    by 0x40371F: heap::deallocate::h7bc897bb314e6fc3Bfa::v0.0 (in /home/alex/foo)
==9880==    by 0x403699: heap::exchange_free::h3a6062ae495b91fcefa::v0.0 (in /home/alex/foo)
==9880==    by 0x403FA0: _$UP$i8::glue_drop.1534::h08c160eb1ce0fd99 (in /home/alex/foo)
==9880==    by 0x4031D4: main::hfd2b07b8610388fafaa::v0.0 (in /home/alex/foo)
==9880==    by 0x447512: start::closure.7199 (in /home/alex/foo)
==9880==    by 0x463862: task::Task::run::closure.5303 (in /home/alex/foo)
==9880==    by 0x466C4B: rust_try (in /home/alex/foo)
==9880==    by 0x465565: unwind::try::h7d720262cba71fd0fGd::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x463714: task::Task::run::hac578e5b353eedcbVVc::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x4472CD: start::h0149aeff7c3a4b69Jme::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x447098: lang_start::h3e00b03c17efac073le::v0.11.0.pre (in /home/alex/foo)
==9880== 
==9880== Invalid write of size 8
==9880==    at 0x403B4A: cell::Cell$LT$T$GT$::set::h11020375487951902910::v0.0 (in /home/alex/foo)
==9880==    by 0x403DA8: cell::Ref$LT$$x27b$C$$x20T$GT$.Drop::drop::h5517615880807730945::v0.0 (in /home/alex/foo)
==9880==    by 0x40406A: core..cell..Ref$LT$$x27_$C$int$GT$::glue_drop.1537::h7a74728680c53b3a (in /home/alex/foo)
==9880==    by 0x4031E1: main::hfd2b07b8610388fafaa::v0.0 (in /home/alex/foo)
==9880==    by 0x447512: start::closure.7199 (in /home/alex/foo)
==9880==    by 0x463862: task::Task::run::closure.5303 (in /home/alex/foo)
==9880==    by 0x466C4B: rust_try (in /home/alex/foo)
==9880==    by 0x465565: unwind::try::h7d720262cba71fd0fGd::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x463714: task::Task::run::hac578e5b353eedcbVVc::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x4472CD: start::h0149aeff7c3a4b69Jme::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x447098: lang_start::h3e00b03c17efac073le::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x40332E: main (in /home/alex/foo)
==9880==  Address 0x6014058 is 8 bytes inside a block of size 16 free'd
==9880==    at 0x471379: je_dallocx (in /home/alex/foo)
==9880==    by 0x40371F: heap::deallocate::h7bc897bb314e6fc3Bfa::v0.0 (in /home/alex/foo)
==9880==    by 0x403699: heap::exchange_free::h3a6062ae495b91fcefa::v0.0 (in /home/alex/foo)
==9880==    by 0x403FA0: _$UP$i8::glue_drop.1534::h08c160eb1ce0fd99 (in /home/alex/foo)
==9880==    by 0x4031D4: main::hfd2b07b8610388fafaa::v0.0 (in /home/alex/foo)
==9880==    by 0x447512: start::closure.7199 (in /home/alex/foo)
==9880==    by 0x463862: task::Task::run::closure.5303 (in /home/alex/foo)
==9880==    by 0x466C4B: rust_try (in /home/alex/foo)
==9880==    by 0x465565: unwind::try::h7d720262cba71fd0fGd::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x463714: task::Task::run::hac578e5b353eedcbVVc::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x4472CD: start::h0149aeff7c3a4b69Jme::v0.11.0.pre (in /home/alex/foo)
==9880==    by 0x447098: lang_start::h3e00b03c17efac073le::v0.11.0.pre (in /home/alex/foo)
==9880== 
5
==9880== 
==9880== HEAP SUMMARY:
==9880==     in use at exit: 0 bytes in 0 blocks
==9880==   total heap usage: 18 allocs, 18 frees, 2,016 bytes allocated
==9880== 
==9880== All heap blocks were freed -- no leaks are possible
==9880== 
==9880== For counts of detected and suppressed errors, rerun with: -v
==9880== ERROR SUMMARY: 2 errors from 2 contexts (suppressed: 0 from 0)
