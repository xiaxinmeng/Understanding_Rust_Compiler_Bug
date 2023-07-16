
---- [run-pass-valgrind] run-pass-valgrind/coerce-match-calls.rs stdout ----

error: test run failed!
status: exit code: 100
command: /usr/bin/valgrind --error-exitcode=100 --fair-sched=try --quiet --soname-synonyms=somalloc=NONE --suppressions=/buildslave/rust-buildbot/slave/auto-linux-64-opt/build/src/etc/x86.supp --tool=memcheck --leak-check=full x86_64-unknown-linux-gnu/test/run-pass-valgrind/coerce-match-calls.stage2-x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
==20600== Syscall param read(buf) points to unaddressable byte(s)
==20600==    at 0x401B177: read (syscall-template.S:84)
==20600==    by 0x40059FF: open_verify.constprop.7 (dl-load.c:1949)
==20600==    by 0x4005CD9: open_path (dl-load.c:2066)
==20600==    by 0x4008BE0: _dl_map_object (dl-load.c:2307)
==20600==    by 0x400D9D1: openaux (dl-deps.c:63)
==20600==    by 0x4010393: _dl_catch_error (dl-error.c:187)
==20600==    by 0x400E011: _dl_map_object_deps (dl-deps.c:254)
==20600==    by 0x40034F9: dl_main (rtld.c:1610)
==20600==    by 0x4019461: _dl_sysdep_start (dl-sysdep.c:249)
==20600==    by 0x4004E79: _dl_start_final (rtld.c:307)
==20600==    by 0x4004E79: _dl_start (rtld.c:413)
==20600==    by 0x4000CC7: ??? (in /lib/x86_64-linux-gnu/ld-2.23.so)
==20600==  Address 0xffee10a00 is on thread 1's stack
==20600==  in frame #1, created by open_verify.constprop.7 (dl-load.c:1695)
==20600== 

------------------------------------------
