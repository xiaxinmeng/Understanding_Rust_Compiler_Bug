plain
[00:01:51] done.
[00:01:51] Processing triggers for systemd (229-4ubuntu21.4) ...
[00:01:58]  ---> 528b3e1a010d
[00:01:59] Removing intermediate container 82c12a2a3dbc
[00:01:59] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:01:59]  ---> Running in 254cd02dbf6e
[00:01:59] --2018-09-06 12:44:44--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
[00:01:59] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:01:59] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:01:59] HTTP request sent, awaiting response... 200 OK
[00:01:59] Length: 37338817 (36M) [application/x-gzip]
[00:01:59] Saving to: 'gdb-8.1.1.tar.gz'
[00:01:59]      0K .......... .......... .......... .......... ..........  0%  760K 48s
[00:01:59]     50K .......... .......... .......... .......... ..........  0% 1.47M 36s
[00:01:59]    100K .......... .......... .......... .......... ..........  0% 59.5M 24s
[00:01:59]    150K .......... .......... .......... .......... ..........  0% 1.50M 24s
---
[00:02:00]  36350K .......... .......... .......... .......... .......... 99%  146M 0s
[00:02:00]  36400K .......... .......... .......... .......... .......... 99% 75.3M 0s
[00:02:00]  36450K .......... ...                                        100% 69.0M=1.3s
[00:02:00] 
[00:02:00] 2018-09-06 12:44:45 (27.0 MB/s) - 'gdb-8.1.1.tar.gz' saved [37338817/37338817]
[00:02:00] 
[00:02:37] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:16] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:21] complete.c: In function 'fnwidth':
[00:03:21] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:21]         w = wcwidth (wc);
[00:03:21]             ^
[00:03:22] display.c: In function 'rl_redisplay':
[00:03:22] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:22]         temp = wcwidth (wc);
[00:03:22]                ^
[00:03:23] util.c: In function '_rl_tropen':
[00:03:23] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:03:23]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:03:23]                    ^
[00:03:25] histfile.c: In function 'history_truncate_file':
[00:03:25] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:03:25]        write (file, bp, chars_read - (bp - buffer));
[00:03:25]        ^
[00:03:25] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:03:25] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:25]         if (wcwidth (wc) == 0)
[00:03:25]             ^
[00:03:52] configure: WARNING: no enhanced curses library found; disabling TUI
[00:03:53] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:03:53] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:03:53] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:03:53] configure: WARNING: pkg-config not found, guile support disabled
[00:03:53] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:04:00] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:04:32] Creating observer.htmp
[00:04:32] Creating observer.itmp
[00:05:03] cli/cli-cmds.c: In function 'void complete_command(const char*, int)':
[00:05:03] cli/cli-cmds.c:304:48: warning: 'word' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:05:03]          get_max_completions_reached_message ());
[00:05:03]                                                 ^
[00:05:03] cli/cli-cmds.c:277:71: warning: 'tracker' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:05:03]      = tracker->build_completion_result (word, word - arg, strlen (arg));
[00:05:03]                                                                        ^
[00:06:13] breakpoint.c: In function 'void check_status_watchpoint(bpstat)':
[00:06:13] breakpoint.c:5079:4: warning: 'e' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:06:13]     switch (e)
[00:06:13]     ^
[00:06:13] breakpoint.c:5056:20: note: 'e' was declared here
[00:06:13]     wp_check_result e;
[00:07:30] In file included from inferior.h:48:0,
[00:07:30] In file included from inferior.h:48:0,
[00:07:30]                  from infrun.c:26:
[00:07:30] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:07:30] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:07:30]    { set_current_program_space (m_saved_pspace); }
[00:07:30]                                                ^
[00:07:30] infrun.c:927:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:07:30]       maybe_restore_inferior;
[00:07:30]       ^
[00:10:14] ./gdb.texinfo:2021: warning: @xref node name should not contain `.'
[00:10:14] ./gdb.texinfo:8679: warning: @pxref node name should not contain `.'
[00:10:14] ./gdb.texinfo:19385: warning: @ref node name should not contain `.'
[00:10:14] ./python.texi:4019: warning: @ref node name should not contain `.'
[00:10:14] ./python.texi:4055: warning: @ref node name should not contain `.'
[00:10:16] GNU gdb (GDB) 8.1.1
[00:10:16] Copyright (C) 2018 Free Software Foundation, Inc.
[00:10:16] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:10:16] This is free software: you are free to change and redistribute it.
[00:10:16] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[00:10:16] and "show warranty" for details.
[00:10:16] This GDB was configured as "x86_64-pc-linux-gnu".
[00:10:16] Type "show configuration" for configuration details.
[00:10:16] For bug reporting instructions, please see:
[00:10:16] <http://www.gnu.org/software/gdb/bugs/>.
[00:10:16] Find the GDB manual and other documentation resources online at:
[00:10:16] <http://www.gnu.org/software/gdb/documentation/>.
[00:10:16] For help, type "help".
[00:10:16] Type "apropos word" to search for commands related to "word".
[00:10:24] Removing intermediate container 254cd02dbf6e
[00:10:24] Step 4/7 : COPY scripts/sccache.sh /scripts/
[00:10:24]  ---> bb3234f77264
[00:10:24] Step 5/7 : RUN sh /scripts/sccache.sh
---
[00:10:27] Successfully tagged rust-ci:latest
[00:10:28] Built container sha256:79fa336ae6e14b625125506e0045637540c7c747f9cf2d3151ffd078db1ca7be
[00:10:28] Uploading finished image to s3://rust-lang-ci-sccache2/docker/3c8630e76be77fa21087dfa7f6487998dcb30f7d24969ea9c136e6d50a2d1f2e567b29ea7e9bfb96a1fde3c0193d8d82986ea555ff8db1c15bf8098451917a69
[00:10:28] 
[00:10:28] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:10:41] xargs: docker: terminated by signal 13

[00:10:56] travis_time:end:2b249ac0:start=1536237822033495071,finish=1536238406920619154,duration=584887124083
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:10:56] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---

[00:54:09] travis_fold:start:test_ui
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:09] version from s "/usr/local/bin/gdb"
[00:54:09] version from --version Some("GNU gdb (GDB) 8.1.1")
[00:54:09] gdb Some("/usr/local/bin/gdb")
[00:54:09] gdb_version Some(8001001)
[00:54:09] gdb_native_rust true
[00:54:09] running 4201 tests
[00:54:11] ....................................................................................................
[00:54:14] ....................................................................................................
[00:54:17] ....................................................................................................
---
[00:56:14] 
[00:56:14] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:14] version from s "/usr/local/bin/gdb"
[00:56:14] version from --version Some("GNU gdb (GDB) 8.1.1")
[00:56:14] gdb Some("/usr/local/bin/gdb")
[00:56:14] gdb_version Some(8001001)
[00:56:14] gdb_native_rust true
[00:56:14] running 4201 tests
[00:56:17] ...................................................................................................i
[00:56:19] ....................................................................................................
[00:56:22] .................................................................i...i................ii............
---
[00:57:08] ....................................................................................................
[00:57:11] ....................................................................................................
[00:57:14] ................................i...................................................................
[00:57:17] ....................................................................................................
[00:57:20] .................................................................................iiiiiiiii..........
[00:57:25] ....ii..............................................................................................
[00:57:29] ....................................................................................................
[00:57:32] ..............................................................i.....................................
[00:57:34] ....................................................................................................
---
[00:58:04] .......................................................................................i............
[00:58:07] ....................................................................................................
[00:58:10] ....................................................................................................
[00:58:13] ....................................................................................................
[00:58:15] .i.ii.ii.ii.............................i...........................................................
[00:58:15] test result: ok. 4137 passed; 0 failed; 64 ignored; 0 measured; 0 filtered out
[00:58:15] 
[00:58:15]  finished in 121.494
[00:58:15] travis_fold:end:test_ui_nll
---

[00:58:15] travis_fold:start:test_run-pass
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:15] version from s "/usr/local/bin/gdb"
[00:58:15] version from --version Some("GNU gdb (GDB) 8.1.1")
[00:58:15] gdb Some("/usr/local/bin/gdb")
[00:58:15] gdb_version Some(8001001)
[00:58:15] gdb_native_rust true
[00:58:15] Start "/checkout/src/test/run-pass/issue24687-embed-debuginfo/main.rs"
[00:58:15] Start "/checkout/src/test/run-pass/backtrace-debuginfo.rs"
[00:58:16] Start "/checkout/src/test/run-pass/backtrace-debuginfo-aux.rs"
[00:58:16] false gdb
[00:58:16] false gdb
[00:58:16] "/checkout/src/test/run-pass/backtrace-debuginfo-aux.rs" will be ignored
[00:58:16] Start "/checkout/src/test/run-pass/debuginfo-lto.rs"
[00:58:16] running 3084 tests
[00:58:24] ....................................................................................................
[00:58:34] ...............................................i....................................................
[00:58:45] ....................................................................................................
---

[01:03:48] travis_fold:start:test_compile-fail
travis_time:start:test_compile-fail
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:48] version from s "/usr/local/bin/gdb"
[01:03:48] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:03:48] gdb Some("/usr/local/bin/gdb")
[01:03:48] gdb_version Some(8001001)
[01:03:48] gdb_native_rust true
[01:03:48] running 21 tests
[01:03:49] .....................
[01:03:49] test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:49] 
---

[01:03:49] travis_fold:start:test_parse-fail
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:49] version from s "/usr/local/bin/gdb"
[01:03:49] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:03:49] gdb Some("/usr/local/bin/gdb")
[01:03:49] gdb_version Some(8001001)
[01:03:49] gdb_native_rust true
[01:03:49] running 274 tests
[01:03:50] ..............................i.....................................................................
[01:03:52] .......................................................................i............................
[01:03:53] ..................i.......................................................
---

[01:03:53] travis_fold:start:test_run-fail
travis_time:start:test_run-fail
Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:53] version from s "/usr/local/bin/gdb"
[01:03:53] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:03:53] gdb Some("/usr/local/bin/gdb")
[01:03:53] gdb_version Some(8001001)
[01:03:53] gdb_native_rust true
[01:03:53] running 143 tests
[01:03:59] ....................................................................................................
[01:04:03] ...........................................
[01:04:03] test result: ok. 143 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
---

[01:04:03] travis_fold:start:test_run-pass-valgrind
travis_time:start:test_run-pass-valgrind
Check compiletest suite=run-pass-valgrind mode=run-pass-valgrind (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:03] version from s "/usr/local/bin/gdb"
[01:04:03] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:04:03] gdb Some("/usr/local/bin/gdb")
[01:04:03] gdb_version Some(8001001)
[01:04:03] gdb_native_rust true
[01:04:03] running 14 tests
[01:04:05] ..............
[01:04:05] test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:05] 
---

[01:04:05] travis_fold:start:test_mir-opt
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:05] version from s "/usr/local/bin/gdb"
[01:04:05] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:04:05] gdb Some("/usr/local/bin/gdb")
[01:04:05] gdb_version Some(8001001)
[01:04:05] gdb_native_rust true
[01:04:05] running 46 tests
[01:04:23] ..............................................
[01:04:23] test result: ok. 46 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:23] 
---

[01:04:23] travis_fold:start:test_codegen
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:23] version from s "/usr/local/bin/gdb"
[01:04:23] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:04:23] gdb Some("/usr/local/bin/gdb")
[01:04:23] gdb_version Some(8001001)
[01:04:23] gdb_native_rust true
[01:04:23] running 106 tests
[01:04:26] i..ii..iii....i...i............iii...........i.....i....ii...i.i.ii..............i...ii..ii.i....iii
[01:04:27] i.....
[01:04:27] test result: ok. 77 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out
---

[01:04:27] travis_fold:start:test_codegen-units
travis_time:start:test_codegen-units
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:27] version from s "/usr/local/bin/gdb"
[01:04:27] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:04:27] gdb Some("/usr/local/bin/gdb")
[01:04:27] gdb_version Some(8001001)
[01:04:27] gdb_native_rust true
[01:04:27] running 39 tests
[01:04:28] i.......i......................i.......
[01:04:28] test result: ok. 36 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out
[01:04:28] 
---

[01:04:28] travis_fold:start:test_incremental
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:28] version from s "/usr/local/bin/gdb"
[01:04:28] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:04:28] gdb Some("/usr/local/bin/gdb")
[01:04:28] gdb_version Some(8001001)
[01:04:28] gdb_native_rust true
[01:04:28] Start "/checkout/src/test/incremental/spans_in_type_debuginfo.rs"
[01:04:28] Start "/checkout/src/test/incremental/spans_significant_w_debuginfo.rs"
[01:04:28] running 90 tests
[01:04:40] ..........................................................................................
[01:04:40] test result: ok. 90 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:40] 
---

[01:04:40] travis_fold:start:test_debuginfo
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:40] version from s "/usr/local/bin/gdb"
[01:04:40] version from --version Some("GNU gdb (GDB) 8.1.1")
[01:04:40] gdb Some("/usr/local/bin/gdb")
[01:04:40] gdb_version Some(8001001)
[01:04:40] gdb_native_rust true
[01:04:40] Start "/checkout/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/lexical-scope-in-unconditional-loop.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/by-value-non-immediate-argument.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] "/checkout/src/test/debuginfo/by-value-non-immediate-argument.rs" will be ignored
[01:04:40] Start "/checkout/src/test/debuginfo/pretty-std.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] "/checkout/src/test/debuginfo/pretty-std.rs" will be ignored
[01:04:40] Start "/checkout/src/test/debuginfo/struct-namespace.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] "/checkout/src/test/debuginfo/struct-namespace.rs" will be ignored
[01:04:40] Start "/checkout/src/test/debuginfo/by-value-self-argument-in-trait-impl.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/generic-struct-style-enum.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/var-captured-in-sendable-closure.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/extern-c-fn.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/recursive-enum.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/basic-types-globals.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] "/checkout/src/test/debuginfo/basic-types-globals.rs" will be ignored
[01:04:40] Start "/checkout/src/test/debuginfo/trait-pointers.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/pretty-uninitialized-vec.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/macro-stepping.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] "/checkout/src/test/debuginfo/macro-stepping.rs" will be ignored
[01:04:40] Start "/checkout/src/test/debuginfo/type-names.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] "/checkout/src/test/debuginfo/type-names.rs" will be ignored
[01:04:40] Start "/checkout/src/test/debuginfo/multi-cgu.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/self-in-default-method.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/unreachable-locals.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/no-debug-attribute.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/pretty-std-collections.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/var-captured-in-nested-closure.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/simple-lexical-scope.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/destructured-fn-argument.rs"
[01:04:40] Start "/checkout/src/test/debuginfo/simple-struct.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] "/checkout/src/test/debuginfo/simple-struct.rs" will be ignored
[01:04:40] Start "/checkout/src/test/debuginfo/drop-locations.rs"
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
[01:04:40] false gdb
---
[01:04:47] test result: FAILED. 82 passed; 5 failed; 24 ignored; 0 measured; 0 filtered out
[01:04:47] 
[01:04:47] 
[01:04:47] 
[01:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/local/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:47] 
[01:04:47] 
[01:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:47] Build completed unsuccessfully in 0:11:40
[01:04:47] Build completed unsuccessfully in 0:11:40
[01:04:47] Makefile:58: recipe for target 'check' failed
[01:04:47] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f592240
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0615b154:start=1536241654429803402,finish=1536241654436423139,duration=6619737
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:119129de
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:106c6174
travis_time:start:106c6174
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0538fe9a
$ dmesg | grep -i kill
