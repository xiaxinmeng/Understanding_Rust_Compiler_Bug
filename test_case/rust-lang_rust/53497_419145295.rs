plain
[00:02:26] done.
[00:02:26] Processing triggers for systemd (229-4ubuntu21.4) ...
[00:02:34]  ---> 3c244f66b4fa
[00:02:34] Removing intermediate container e32dd36e3260
[00:02:34] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.2.tar.gz > /dev/null &&   tar -xzvf gdb-8.2.tar.gz > /dev/null &&   cd gdb-8.2 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:02:34]  ---> Running in 2ef5b5872176
[00:02:34] --2018-09-06 14:44:22--  http://ftp.gnu.org/gnu/gdb/gdb-8.2.tar.gz
[00:02:34] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:02:34] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:02:34] HTTP request sent, awaiting response... 200 OK
[00:02:34] Length: 37605479 (36M) [application/x-gzip]
[00:02:34] Saving to: 'gdb-8.2.tar.gz'
[00:02:35]      0K .......... .......... .......... .......... ..........  0%  310K 1m58s
[00:02:35]     50K .......... .......... .......... .......... ..........  0%  579K 91s
[00:02:35]    100K .......... .......... .......... .......... ..........  0% 57.7M 61s
[00:02:35]    150K .......... .......... .......... .......... ..........  0%  477K 65s
---
[00:02:53]  36600K .......... .......... .......... .......... .......... 99% 3.14M 0s
[00:02:53]  36650K .......... .......... .......... .......... .......... 99% 28.7M 0s
[00:02:53]  36700K .......... .......... ....                            100% 6.23M=19s
[00:02:53] 
[00:02:53] 2018-09-06 14:44:41 (1.90 MB/s) - 'gdb-8.2.tar.gz' saved [37605479/37605479]
[00:02:53] 
[00:03:31] ar: `u' modifier ignored since `D' is the default (see `U')
[00:04:07] ar: `u' modifier ignored since `D' is the default (see `U')
[00:04:13] complete.c: In function 'fnwidth':
[00:04:13] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:04:13]         w = wcwidth (wc);
[00:04:13]             ^
[00:04:14] display.c: In function 'rl_redisplay':
[00:04:14] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:04:14]         temp = wcwidth (wc);
[00:04:14]                ^
[00:04:15] util.c: In function '_rl_tropen':
[00:04:15] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:04:15]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:04:15]                    ^
[00:04:17] histfile.c: In function 'history_truncate_file':
[00:04:17] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:04:17]        write (file, bp, chars_read - (bp - buffer));
[00:04:17]        ^
[00:04:17] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:04:17] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:04:17]         if (wcwidth (wc) == 0)
[00:04:17]             ^
[00:04:44] configure: WARNING: no enhanced curses library found; disabling TUI
[00:04:44] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:04:44] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:04:44] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:04:44] configure: WARNING: pkg-config not found, guile support disabled
[00:04:44] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:04:52] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:08:47] In file included from inferior.h:49:0,
[00:08:47]                  from infrun.c:26:
[00:08:47] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:08:47] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:08:47]    { set_current_program_space (m_saved_pspace); }
[00:08:47]                                                ^
[00:08:47] infrun.c:929:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:08:47]       maybe_restore_inferior;
[00:08:47]       ^
[00:11:55] ./gdb.texinfo:2021: warning: @xref node name should not contain `.'
[00:11:55] ./gdb.texinfo:8757: warning: @pxref node name should not contain `.'
[00:11:55] ./gdb.texinfo:19508: warning: @ref node name should not contain `.'
[00:11:55] ./python.texi:4060: warning: @ref node name should not contain `.'
[00:11:55] ./python.texi:4096: warning: @ref node name should not contain `.'
[00:11:58] GNU gdb (GDB) 8.2
[00:11:58] Copyright (C) 2018 Free Software Foundation, Inc.
[00:11:58] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:11:58] This is free software: you are free to change and redistribute it.
[00:12:06]  ---> 8c40a60be255
[00:12:07] Removing intermediate container 2ef5b5872176
[00:12:07] Step 4/7 : COPY scripts/sccache.sh /scripts/
[00:12:07]  ---> f612abdba9a1
---
[00:12:11] Successfully tagged rust-ci:latest
[00:12:11] Built container sha256:68e51cdd466ebbab91f0e7e2de2d259eee011249a6a69b0520a23de41009ea1e
[00:12:11] Uploading finished image to s3://rust-lang-ci-sccache2/docker/6eb5e791a159ed80b8a9ce11b4a913cde54f55eec7345b68d034d23b7696a4de1a0b516662832917b42a84385c9c5eeecb54560d5ebb793083a2ccd7c3ce6e11
[00:12:11] 
[00:12:11] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:12:28] xargs: docker: terminated by signal 13

[00:13:04] travis_time:end:04e3f1d0:start=1536244996960166094,finish=1536245656232473217,duration=659272307123
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:13:04] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:59:22] ....................................................................................................
[00:59:26] ....................................................................................................
[00:59:28] ................................i...................................................................
[00:59:31] ....................................................................................................
[00:59:34] .................................................................................iiiiiiiii..........
[00:59:39] ...ii...............................................................................................
[00:59:43] ....................................................................................................
[00:59:46] ..............................................................i.....................................
[00:59:49] ....................................................................................................
---
[01:00:19] .......................................................................................i............
[01:00:22] ....................................................................................................
[01:00:26] ....................................................................................................
[01:00:28] ....................................................................................................
[01:00:31] .i.ii.ii.ii.............................i...........................................................
[01:00:31] test result: ok. 4137 passed; 0 failed; 64 ignored; 0 measured; 0 filtered out
[01:00:31] 
[01:00:31]  finished in 123.748
[01:00:31] travis_fold:end:test_ui_nll
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:33] 
[01:07:33] running 111 tests
[01:07:39] iiii.......i..i........i..i..i..FF.....F.F.i..........iiii...........i.F..i.FF....F..ii.i.i.......ii
[01:07:39] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:07:39] failures:
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/generic-enum-with-different-disr-sizes.rs stdout ----
[01:07:39] ---- [debuginfo-gdb] debuginfo/generic-enum-with-different-disr-sizes.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = generic_enum_with_different_disr_sizes::Enum::Variant1(100)
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generic-enum-with-different-disr-sizes/generic-enum-with-different-disr-sizes.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0x9fa: file /checkout/src/test/debuginfo/generic-enum-with-different-disr-sizes.rs, line 105.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, generic_enum_with_different_disr_sizes::main () at /checkout/src/test/debuginfo/generic-enum-with-different-disr-sizes.rs:105
[01:07:39] 105     zzz(); // #break
[01:07:39] $1 = generic_enum_with_different_disr_sizes::Enum<f64>::Variant1(100)
[01:07:39] $2 = generic_enum_with_different_disr_sizes::Enum<i32>::Variant1(101)
[01:07:39] $3 = generic_enum_with_different_disr_sizes::Enum<i16>::Variant1(102)
[01:07:39] $4 = generic_enum_with_different_disr_sizes::Enum<u8>::Variant1(65)
[01:07:39] $5 = generic_enum_with_different_disr_sizes::Enum<f64>::Variant2(100)
[01:07:39] $6 = generic_enum_with_different_disr_sizes::Enum<i32>::Variant2(101)
[01:07:39] $7 = generic_enum_with_different_disr_sizes::Enum<i16>::Variant2(102)
[01:07:39] $8 = generic_enum_with_different_disr_sizes::Enum<u8>::Variant2(65)
[01:07:39] [Inferior 1 (process 15037) exited normally]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/generic-enum-with-different-disr-sizes.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7/gdb-pretty-struct-and-enums-pre-gdb-7-7.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0xa8f: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs, line 70.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, gdb_pretty_struct_and_enums_pre_gdb_7_7::main () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs:70
[01:07:39] 70     zzz(); // #break
[01:07:39] $1 = gdb_pretty_struct_and_enums_pre_gdb_7_7::RegularStruct {the_first_field: 101, the_second_field: 102.5, the_third_field: false}
[01:07:39] $2 = gdb_pretty_struct_and_enums_pre_gdb_7_7::EmptyStruct
[01:07:39] $3 = gdb_pretty_struct_and_enums_pre_gdb_7_7::CStyleEnum::CStyleEnumVar1
[01:07:39] $4 = gdb_pretty_struct_and_enums_pre_gdb_7_7::CStyleEnum::CStyleEnumVar2
[01:07:39] $5 = gdb_pretty_struct_and_enums_pre_gdb_7_7::CStyleEnum::CStyleEnumVar3
[01:07:39] A debugging session is active.
[01:07:39] 
[01:07:39]  Inferior 1 [process 15033] will be killed.
[01:07:39] 
[01:07:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
[01:07:39] of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7/a.
[01:07:39] Use `info auto-load python-scripts [REGEXP]' to list them.
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] 
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/generic-struct-style-enum.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = generic_struct_style_enum::Regular::Case1{a: 0, b: 31868, c: 31868, d: 31868, e: 31868}
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generic-struct-style-enum/generic-struct-style-enum.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0x9f7: file /checkout/src/test/debuginfo/generic-struct-style-enum.rs, line 86.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, generic_struct_style_enum::main () at /checkout/src/test/debuginfo/generic-struct-style-enum.rs:86
[01:07:39] 86     zzz(); // #break
[01:07:39] $1 = generic_struct_style_enum::Regular<u16, u32, i64>::Case1{a: 0, b: 31868, c: 31868, d: 31868, e: 31868}
[01:07:39] $2 = generic_struct_style_enum::Regular<i16, u32, i64>::Case2{a: 0, b: 286331153, c: 286331153}
[01:07:39] $3 = generic_struct_style_enum::Regular<u16, i32, u64>::Case3{a: 0, b: 6438275382588823897}
[01:07:39] $4 = generic_struct_style_enum::Univariant<i32>::TheOnlyCase{a: -1}
[01:07:39] A debugging session is active.
[01:07:39] 
[01:07:39]  Inferior 1 [process 15158] will be killed.
[01:07:39] 
[01:07:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/generic-struct-style-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/generic-tuple-style-enum.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = generic_tuple_style_enum::Regular::Case1(0, 31868, 31868, 31868, 31868)
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generic-tuple-style-enum/generic-tuple-style-enum.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0x9f8: file /checkout/src/test/debuginfo/generic-tuple-style-enum.rs, line 104.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, generic_tuple_style_enum::main () at /checkout/src/test/debuginfo/generic-tuple-style-enum.rs:104
[01:07:39] 104     zzz(); // #break
[01:07:39] $1 = generic_tuple_style_enum::Regular<u16, u32, u64>::Case1(0, 31868, 31868, 31868, 31868)
[01:07:39] $2 = generic_tuple_style_enum::Regular<i16, i32, i64>::Case2(0, 286331153, 286331153)
[01:07:39] $3 = generic_tuple_style_enum::Regular<i16, i32, i64>::Case3(0, 6438275382588823897)
[01:07:39] $4 = generic_tuple_style_enum::Univariant<i64>::TheOnlyCase(-1)
[01:07:39] A debugging session is active.
[01:07:39] 
[01:07:39]  Inferior 1 [process 15197] will be killed.
[01:07:39] 
[01:07:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/generic-tuple-style-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = <error reading variable>
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/nil-enum/nil-enum.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0x9de: file /checkout/src/test/debuginfo/nil-enum.rs, line 41.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, nil_enum::main () at /checkout/src/test/debuginfo/nil-enum.rs:41
[01:07:39] 41         zzz(); // #break
[01:07:39] $1 = nil_enum::ANilEnum
[01:07:39] $2 = nil_enum::AnotherNilEnum
[01:07:39] A debugging session is active.
[01:07:39] 
[01:07:39]  Inferior 1 [process 15672] will be killed.
[01:07:39] 
[01:07:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/nil-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/pretty-huge-vec.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/pretty-huge-vec.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0x173c: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 38.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, pretty_huge_vec::main () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:38
[01:07:39] 38     zzz(); // #break
[01:07:39] $1 = alloc::vec::Vec<u8> {buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global> {ptr: core::ptr::Unique<u8> {pointer: core::nonzero::NonZero<*const u8> (0x7fffb6a00000 "\000"), _marker: core::marker::PhantomData<u8>}, cap: 1000000000, a: alloc::alloc::Global}, len: 1000000000}
[01:07:39] $2 = &[u8] {data_ptr: 0x7fffb6a00000 "\000", length: 1000000000}
[01:07:39] A debugging session is active.
[01:07:39] 
[01:07:39]  Inferior 1 [process 15761] will be killed.
[01:07:39] 
[01:07:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
[01:07:39] of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/a.
[01:07:39] Use `info auto-load python-scripts [REGEXP]' to list them.
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/pretty-huge-vec.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] 
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/pretty-uninitialized-vec.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = Vec<i32>(len: [...], cap: [...])[...]
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0x12b0: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 31.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, pretty_uninitialized_vec::main () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:31
[01:07:39] 31     zzz(); // #break
[01:07:39] $1 = alloc::vec::Vec<i32> {buf: alloc::raw_vec::RawVec<i32, alloc::alloc::Global> {ptr: core::ptr::Unique<i32> {pointer: core::nonzero::NonZero<*const i32> (0x555555555920 <__libc_csu_init>), _marker: core::marker::PhantomData<i32>}, cap: 93824992234032, a: alloc::alloc::Global}, len: 140737488348624}
[01:07:39] A debugging session is active.
[01:07:39] 
[01:07:39]  Inferior 1 [process 15815] will be killed.
[01:07:39] 
[01:07:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
[01:07:39] of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/a.
[01:07:39] Use `info auto-load python-scripts [REGEXP]' to list them.
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/pretty-uninitialized-vec.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] 
[01:07:39] 
[01:07:39] ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
[01:07:39] NOTE: compiletest thinks it is using GDB with native rust support
[01:07:39] NOTE: compiletest thinks it is using GDB version 8002000
[01:07:39] 
[01:07:39] error: line not found in debugger output: $1 = BTreeSet<i32>(len: 3) = {3, 5, 7}
[01:07:39] status: exit code: 0
[01:07:39] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/pretty-std-collections.debugger.script"
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] GNU gdb (GDB) 8.2
[01:07:39] Copyright (C) 2018 Free Software Foundation, Inc.
[01:07:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:07:39] This is free software: you are free to change and redistribute it.
[01:07:39] There is NO WARRANTY, to the extent permitted by law.
[01:07:39] Type "show copying" and "show warranty" for details.
[01:07:39] This GDB was configured as "x86_64-pc-linux-gnu".
[01:07:39] Type "show configuration" for configuration details.
[01:07:39] For bug reporting instructions, please see:
[01:07:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:07:39] Find the GDB manual and other documentation resources online at:
[01:07:39]     <http://www.gnu.org/software/gdb/documentation/>.
[01:07:39] 
[01:07:39] For help, type "help".
[01:07:39] Type "apropos word" to search for commands related to "word".
[01:07:39] Breakpoint 1 at 0x13d46: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 57.
[01:07:39] [Thread debugging using libthread_db enabled]
[01:07:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:07:39] 
[01:07:39] Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:57
[01:07:39] 57     zzz(); // #break
[01:07:39] $1 = alloc::collections::btree::set::BTreeSet<i32> {map: alloc::collections::btree::map::BTreeMap<i32, ()> {root: alloc::collections::btree::node::Root<i32, ()> {node: alloc::collections::btree::node::BoxedNode<i32, ()> {ptr: core::ptr::Unique<alloc::collections::btree::node::LeafNode<i32, ()>> {pointer: core::nonzero::NonZero<*const alloc::collections::btree::node::LeafNode<i32, ()>> (0x7ffff6a2a000), _marker: core::marker::PhantomData<alloc::collections::btree::node::LeafNode<i32, ()>>}}, height: 0}, length: 3}}
[01:07:39] $2 = alloc::collections::btree::map::BTreeMap<i32, i32> {root: alloc::collections::btree::node::Root<i32, i32> {node: alloc::collections::btree::node::BoxedNode<i32, i32> {ptr: core::ptr::Unique<alloc::collections::btree::node::LeafNode<i32, i32>> {pointer: core::nonzero::NonZero<*const alloc::collections::btree::node::LeafNode<i32, i32>> (0x7ffff6a2b000), _marker: core::marker::PhantomData<alloc::collections::btree::node::LeafNode<i32, i32>>}}, height: 0}, length: 3}
[01:07:39] $3 = alloc::collections::vec_deque::VecDeque<i32> {tail: 0, head: 3, buf: alloc::raw_vec::RawVec<i32, alloc::alloc::Global> {ptr: core::ptr::Unique<i32> {pointer: core::nonzero::NonZero<*const i32> (0x7ffff6a20020), _marker: core::marker::PhantomData<i32>}, cap: 8, a: alloc::alloc::Global}}
[01:07:39] A debugging session is active.
[01:07:39] 
[01:07:39]  Inferior 1 [process 15909] will be killed.
[01:07:39] 
[01:07:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] ------------------------------------------
[01:07:39] warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
[01:07:39] of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/a.
[01:07:39] Use `info auto-load python-scripts [REGEXP]' to list them.
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[debuginfo-gdb] debuginfo/pretty-std-collections.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:07:39] 
---
[01:07:39] test result: FAILED. 79 passed; 8 failed; 24 ignored; 0 measured; 0 filtered out
[01:07:39] 
[01:07:39] 
[01:07:39] 
[01:07:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/local/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:39] 
[01:07:39] 
[01:07:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:39] Build completed unsuccessfully in 0:12:19
[01:07:39] Build completed unsuccessfully in 0:12:19
[01:07:39] Makefile:58: recipe for target 'check' failed
[01:07:39] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:216995ec
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
