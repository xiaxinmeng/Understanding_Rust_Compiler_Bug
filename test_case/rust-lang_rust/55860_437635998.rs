plain
travis_time:end:25d31282:start=1541895215124864521,finish=1541895277768586461,duration=62643721940
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:51] .................................................................................................... 100/5003
[00:51:54] .................................................................................................... 200/5003
[00:51:57] ........................................................................ii...................ii..... 300/5003
[00:52:00] ...........................................................................................iii...... 400/5003
[00:52:03] ..iiiiiiii.iii............................iii...........................................i........... 500/5003
[00:52:10] .................................................................................................... 700/5003
[00:52:16] ......................................................................i...........i................. 800/5003
[00:52:19] .........................................................................................iiiii...... 900/5003
[00:52:23] .............iiiiii................................................................................. 1000/5003
---
[00:52:59] .................................................................................................... 2200/5003
[00:53:04] .................................................................................................... 2300/5003
[00:53:07] .................................................................................................... 2400/5003
[00:53:11] .................................................................................................... 2500/5003
[00:53:15] .....................................................................iiiiiiiii...................... 2600/5003
[00:53:22] .................................ii................................................................. 2800/5003
[00:53:25] .................................................................................................... 2900/5003
[00:53:29] .................................................................................................... 3000/5003
[00:53:32] ............................i....................................................................... 3100/5003
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:41] 
[01:07:41] running 115 tests
[01:07:44] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:07:45] ..i...iiii.....
[01:07:45] 
[01:07:45]  finished in 3.671
[01:07:45] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:00] 
[01:08:00] running 118 tests
[01:08:25] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:08:29] ......iii.i.....ii
[01:08:29] 
[01:08:29]  finished in 29.509
[01:08:29] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:06] 
[01:15:06] running 273 tests
[01:16:13] .......................i.......FF................................................................... 100/273
[01:17:08] .....F...............F..F..F...i........F.....................F.....F...............F.....F......... 200/273
[01:17:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:17:52] ....................................................F.F..................
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/edition-doctest.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
---
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/edition-doctest.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/edition-flag.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag" "/checkout/src/test/rustdoc/edition-flag.rs" "--test" "-Z" "unstable-options" "--edition=2018"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/edition-flag.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-18199.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199" "/checkout/src/test/rustdoc/issue-18199.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-18199.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-23106.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-23744.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744" "/checkout/src/test/rustdoc/issue-23744.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 2 tests
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: thread 'failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-23744.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-25944.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-30252.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 5 tests
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-38129.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-43153.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-43153.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/issue-48377.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377" "/checkout/src/test/rustdoc/issue-48377.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
---
[01:17:52]    |
[01:17:52] 13 | //! This is a doc comment
[01:17:52]    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:17:52] 
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
---
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] 
[01:17:52] thread '[rustdoc] rustdoc/issue-54478-demo-allocator.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:52] 
[01:17:52] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:17:52] 
[01:17:52] error: rustdoc failed!
[01:17:52] status: signal: 6
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] running 1 test
[01:17:52] 
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
---
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] thread '<unnamed>' panicked at 'no ImplicitCtxt stored in tls', libcore/option.rs:1008:5
[01:17:52] fatal runtime error: failed to initiate panic, error 5
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
---
[01:17:52] test result: FAILED. 258 passed; 13 failed; 2 ignored; 0 measured; 0 filtered out
[01:17:52] 
[01:17:52] 
[01:17:52] 
[01:17:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:52] 
[01:17:52] 
[01:17:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:52] Build completed unsuccessfully in 0:29:49
[01:17:52] Build completed unsuccessfully in 0:29:49
[01:17:52] make: *** [check] Error 1
[01:17:52] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08861538
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 11 01:32:39 UTC 2018
---
travis_time:end:09fec8a7:start=1541899964329437225,finish=1541899964335861351,duration=6424126
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:069d1dc0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.21931.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 21933]
[New LWP 21931]
[New LWP 21932]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f4721a29428 in ?? ()
#0  0x00007f4721a29428 in ?? ()
#1  0x00007f4721a2b02a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.21936.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 21938]
[New LWP 21937]
[New LWP 21936]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f4877aea428 in ?? ()
#0  0x00007f4877aea428 in ?? ()
#1  0x00007f4877aec02a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.22526.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 22532]
[New LWP 22526]
[New LWP 22527]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007ff4bc9f7428 in ?? ()
#0  0x00007ff4bc9f7428 in ?? ()
#1  0x00007ff4bc9f902a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.22675.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 22677]
[New LWP 22675]
[New LWP 22676]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f52e11f4428 in ?? ()
#0  0x00007f52e11f4428 in ?? ()
#1  0x00007f52e11f602a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.22705.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 22708]
[New LWP 22705]
[New LWP 22707]
[New LWP 22706]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f9133a8f428 in ?? ()
#0  0x00007f9133a8f428 in ?? ()
#1  0x00007f9133a9102a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.22718.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 22720]
[New LWP 22718]
[New LWP 22719]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007fce26a06428 in ?? ()
#0  0x00007fce26a06428 in ?? ()
#1  0x00007fce26a0802a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.22813.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 22815]
[New LWP 22813]
[New LWP 22814]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007fa27b40b428 in ?? ()
#0  0x00007fa27b40b428 in ?? ()
#1  0x00007fa27b40d02a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.22974.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 22978]
[New LWP 22974]
[New LWP 22977]
[New LWP 22979]
[New LWP 22975]
[New LWP 22976]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f98c92cb428 in ?? ()
#0  0x00007f98c92cb428 in ?? ()
#1  0x00007f98c92cd02a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.23023.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 23025]
[New LWP 23023]
[New LWP 23024]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
[New LWP 23098]
[New LWP 23096]
[New LWP 23097]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f38acbf6428 in ?? ()
#0  0x00007f38acbf6428 in ?? ()
#1  0x00007f38acbf802a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.23138.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 23140]
[New LWP 23138]
[New LWP 23139]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc -L /checkout/ob'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f9495b36428 in ?? ()
#0  0x00007f9495b36428 in ?? ()
#1  0x00007f9495b3802a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.23448.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 23450]
[New LWP 23448]
[New LWP 23449]
[New LWP 23452]
[New LWP 23451]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libdl.so.2.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
