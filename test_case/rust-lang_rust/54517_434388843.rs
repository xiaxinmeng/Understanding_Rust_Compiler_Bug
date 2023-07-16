plain
travis_time:end:0ca19516:start=1540915242478663175,finish=1540915244772763354,duration=2294100179
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:48:59] .................................................................................................... 100/4983
[00:49:02] .................................................................................................... 200/4983
[00:49:05] ............................................................................................ii...... 300/4983
[00:49:07] .........................................................................................iii........ 400/4983
[00:49:10] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:49:17] .................................................................................................... 700/4983
[00:49:23] ..................................................................i...........i..................... 800/4983
[00:49:26] ....................................................................................iiiii........... 900/4983
[00:49:30] .................................................................................................... 1000/4983
---
[00:50:06] .................................................................................................... 2200/4983
[00:50:10] .................................................................................................... 2300/4983
[00:50:13] .................................................................................................... 2400/4983
[00:50:17] .................................................................................................... 2500/4983
[00:50:20] ...................................................................iiiiiiiii........................ 2600/4983
[00:50:27] ..................ii................................................................................ 2800/4983
[00:50:30] .................................................................................................... 2900/4983
[00:50:34] .................................................................................................... 3000/4983
[00:50:36] .............i...................................................................................... 3100/4983
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:25] 
[01:03:25] running 112 tests
[01:03:28] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:03:28] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:03:28] 
[01:03:28]  finished in 3.525
[01:03:28] travis_fold:end:test_codegen
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:48] 
[01:04:48] running 97 tests
[01:06:44] ..F....................................................F.....test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:08:44] failures:
[01:08:44] 
[01:08:44] ---- [run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs stdout ----
[01:08:44] 
---
[01:08:44] 
[01:08:44] ------------------------------------------
[01:08:44] stderr:
[01:08:44] ------------------------------------------
[01:08:44] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Diagnostic { level: Fatal, message: [("parse error", NoStyle)], code: None, span: MultiSpan { primary_spans: [Span { lo: BytePos(8), hi: BytePos(11), ctxt: #0 }], span_labels: [] }, children: [], suggestions: [] }', libcore/result.rs:1009:5
[01:08:44] 
[01:08:44] ------------------------------------------
[01:08:44] 
[01:08:44] thread '[run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
---
[01:08:44] status: exit code: 101
[01:08:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/a"
[01:08:44] stdout:
[01:08:44] ------------------------------------------
[01:08:44] printed: box box x
[01:08:44] printed: box x()
[01:08:44] ------------------------------------------
[01:08:44] stderr:
[01:08:44] ------------------------------------------
[01:08:44] ------------------------------------------
[01:08:44] thread 'main' panicked at 'exprs are not equal:
[01:08:44]   e =      "(box ((x)()))"
[01:08:44]   parsed = "(box (box (x)))"', /checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs:223:9
[01:08:44] 
[01:08:44] ------------------------------------------
[01:08:44] 
[01:08:44] thread '[run-pass] run-pass-fulldeps/pprust-expr-roundtrip.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
---
[01:08:44] 
[01:08:44] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:08:44] 
[01:08:44] 
[01:08:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:44] 
[01:08:44] 
[01:08:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:44] Build completed unsuccessfully in 0:23:25
[01:08:44] Build completed unsuccessfully in 0:23:25
[01:08:44] Makefile:58: recipe for target 'check' failed
[01:08:44] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:33bc5d1b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:149bea16:start=1540919381382241505,finish=1540919381388611461,duration=6369956
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11f84406
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:064730d8
travis_time:start:064730d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00a974ee
$ dmesg | grep -i kill
