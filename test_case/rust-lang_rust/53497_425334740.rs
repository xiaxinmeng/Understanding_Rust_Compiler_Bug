plain
[00:01:53] done.
[00:01:53] Processing triggers for systemd (229-4ubuntu21.4) ...
[00:02:00]  ---> 253b933c050a
[00:02:00] Removing intermediate container 3d87fa6623fe
[00:02:00] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.2.tar.gz > /dev/null &&   tar -xzvf gdb-8.2.tar.gz > /dev/null &&   cd gdb-8.2 &&   ./configure --disable-werror --disable-binutils --disable-gas --disable-gold --disable-ld --disable-gprof > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:02:00]  ---> Running in 2929e0b9a9e7
[00:02:01] --2018-09-28 05:08:26--  http://ftp.gnu.org/gnu/gdb/gdb-8.2.tar.gz
[00:02:01] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:02:01] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:02:01] HTTP request sent, awaiting response... 200 OK
[00:02:01] Length: 37605479 (36M) [application/x-gzip]
[00:02:01] Saving to: 'gdb-8.2.tar.gz'
[00:02:01]      0K .......... .......... .......... .......... ..........  0%  775K 47s
[00:02:01]     50K .......... .......... .......... .......... ..........  0% 1.51M 36s
[00:02:01]    100K .......... .......... .......... .......... ..........  0%  146M 24s
[00:02:01]    150K .......... .......... .......... .......... ..........  0% 1.54M 24s
---
[00:02:03]  36600K .......... .......... .......... .......... .......... 99% 36.0M 0s
[00:02:03]  36650K .......... .......... .......... .......... .......... 99% 75.1M 0s
[00:02:03]  36700K .......... .......... ....                            100% 52.0M=2.0s
[00:02:03] 
[00:02:03] 2018-09-28 05:08:29 (18.2 MB/s) - 'gdb-8.2.tar.gz' saved [37605479/37605479]
[00:02:03] 
[00:02:04] configure: WARNING: neither ld nor gold are enabled
[00:02:38] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:14] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:20] complete.c: In function 'fnwidth':
[00:03:20] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:20]         w = wcwidth (wc);
[00:03:20]             ^
[00:03:21] display.c: In function 'rl_redisplay':
[00:03:21] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:21]         temp = wcwidth (wc);
[00:03:21]                ^
[00:03:22] util.c: In function '_rl_tropen':
[00:03:22] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:03:22]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:03:22]                    ^
[00:03:24] histfile.c: In function 'history_truncate_file':
[00:03:24] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:03:24]        write (file, bp, chars_read - (bp - buffer));
[00:03:24]        ^
[00:03:24] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:03:24] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:24]         if (wcwidth (wc) == 0)
[00:03:24]             ^
[00:03:50] configure: WARNING: no enhanced curses library found; disabling TUI
[00:03:50] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:03:50] configure: WARNING: pkg-config not found, guile support disabled
[00:03:50] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:03:59] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:08:34] In file included from inferior.h:49:0,
[00:08:34]                  from infrun.c:26:
[00:08:34] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:08:34] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:08:34]    { set_current_program_space (m_saved_pspace); }
[00:08:34]                                                ^
[00:08:34] infrun.c:929:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:08:34]       maybe_restore_inferior;
[00:08:34]       ^
[00:11:50] ./gdb.texinfo:2021: warning: @xref node name should not contain `.'
[00:11:50] ./gdb.texinfo:8757: warning: @pxref node name should not contain `.'
[00:11:50] ./gdb.texinfo:19508: warning: @ref node name should not contain `.'
[00:11:50] ./python.texi:4060: warning: @ref node name should not contain `.'
[00:11:50] ./python.texi:4096: warning: @ref node name should not contain `.'
[00:11:53] GNU gdb (GDB) 8.2
[00:11:53] Copyright (C) 2018 Free Software Foundation, Inc.
[00:11:53] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:11:53] This is free software: you are free to change and redistribute it.
[00:12:02]  ---> b5ea55a79549
[00:12:02] Removing intermediate container 2929e0b9a9e7
[00:12:02] Step 4/7 : COPY scripts/sccache.sh /scripts/
[00:12:02]  ---> da84f985d69e
---
[00:12:06] Successfully tagged rust-ci:latest
[00:12:06] Built container sha256:f242dff716f5c2824e45e3dd9b6d6db04110bedb0e168774232f2283490a8fe8
[00:12:06] Uploading finished image to s3://rust-lang-ci-sccache2/docker/f99dead856dd9c096bedfb6e01ce3e5f477038a700b1382922ed8213a8290654e53a3f367507855a0090041bcc456892998a19d816da6445dbfb142d6b539bd2
[00:12:06] 
[00:12:06] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:12:21] xargs: docker: terminated by signal 13

[00:12:32] travis_time:end:024966e1:start=1538111243352963936,finish=1538111926769259321,duration=683416295385
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:12:32] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[01:04:46] ....................................................................................................
[01:04:50] ...............................................................i....................................
[01:04:52] ....................................................................................................
[01:04:56] ....................................................................................................
[01:04:58] ............iiiiiiiii...............................................................................
[01:05:04] ....................................................................................................
[01:05:08] ................................................................................................i...
[01:05:10] ....................................................................................................
[01:05:13] ........................................................i..i.ii.....................................
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:00] 
[01:19:00] running 111 tests
Type "apropos word" to search for commands related to "word".
[01:19:07] Breakpoint 1 at 0xa27: file /checkout/src/test/debuginfo/generic-struct-style-enum.rs, line 86.
[01:19:07] [Thread debugging using libthread_db enabled]
[01:19:07] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:19:07] 
[01:19:07] Breakpoint 1, generic_struct_style_enum::main () at /checkout/src/test/debuginfo/generic-struct-style-enum.rs:86
[01:19:07] 86     zzz(); // #break
[01:19:07] $1 = generic_struct_style_enum::Regular<u16, u32, i64>::Case1{a: 0, b: 31868, c: 31868, d: 31868, e: 31868}
[01:19:07] $2 = generic_struct_style_enum::Regular<i16, u32, i64>::Case2{a: 0, b: 286331153, c: 286331153}
[01:19:07] $3 = generic_struct_style_enum::Regular<u16, i32, u64>::Case3{a: 0, b: 6438275382588823897}
[01:19:07] $4 = generic_struct_style_enum::Univariant<i32>::TheOnlyCase{a: -1}
[01:19:07] A debugging session is active.
[01:19:07] 
[01:19:07]  Inferior 1 [process 13940] will be killed.
[01:19:07] 
[01:19:07] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:19:07] ------------------------------------------
[01:19:07] stderr:
[01:19:07] ------------------------------------------
[01:19:07] 
[01:19:07] 
[01:19:07] ------------------------------------------
[01:19:07] 
[01:19:07] thread '[debuginfo-gdb] debuginfo/generic-struct-style-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[01:19:07] 
[01:19:07] ---- [debuginfo-gdb] debuginfo/generic-tuple-style-enum.rs stdout ----
[01:19:07] NOTE: compiletest thinks it is using GDB with native rust support
[01:19:07]o.1".
[01:19:07] 
[01:19:07] Breakpoint 1, generic_tuple_style_enum::main () at /checkout/src/test/debuginfo/generic-tuple-style-enum.rs:104
[01:19:07] 104     zzz(); // #break
[01:19:07] $1 = generic_tuple_style_enum::Regular<u16, u32, u64>::Case1(0, 31868, 31868, 31868, 31868)
[01:19:07] $2 = generic_tuple_style_enum::Regular<i16, i32, i64>::Case2(0, 286331153, 286331153)
[01:19:07] $3 = generic_tuple_style_enum::Regular<i16, i32, i64>::Case3(0, 6438275382588823897)
[01:19:07] $4 = generic_tuple_style_enum::Univariant<i64>::TheOnlyCase(-1)
[01:19:07] A debugging session is active.
[01:19:07] 
[01:19:07]  Inferior 1 [process 13974] will be killed.
[01:19:07] 
[01:19:07] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:19:07] ------------------------------------------
[01:19:07] stderr:
[01:19:07] ------------------------------------------
[01:19:07] 
[01:19:07] 
[01:19:07] ------------------------------------------
[01:19:07] 
[01:19:07] thread '[debuginfo-gdb] debuginfo/generic-tuple-style-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[01:19:07] 
[01:19:07] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[01:19:07] NOTE: compiletest thinks it is using GDB with native rust support
[01:19:07] NOTE: compiletest thinks it is using GDB version 8002000
[01:19:07] 
[01:19:07] error: line not found in debugger output: $1 = <error reading variable>
[01:19:07] status: exit code: 0
[01:19:07] command: "/usr/local/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/nil-enum/nil-enum.debugger.script"
[01---------------------------------
[01:19:07] ------------------------------------------
[01:19:07] 
[01:19:07] ------------------------------------------
[01:19:07] 
---
[01:19:07] 
[01:19:07] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:19:07] 
[01:19:07] 
[01:19:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/local/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:07] 
[01:19:07] 
[01:19:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:07] Build completed unsuccessfully in 0:18:50
[01:19:07] Build completed unsuccessfully in 0:18:50
[01:19:07] Makefile:58: recipe for target 'check' failed
[01:19:07] make: *** [check] Error 1
37080 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
37064 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
36376 ./.git/modules/src/libcompiler_builtins
35924 ./obj/build/x86_64-unknown-linux-gnu/test/incremental
---
travis_time:end:001476dc:start=1538115935363658277,finish=1538115935367682756,duration=4024479
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dc177d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then print
