plain
travis_time:end:0fe27de7:start=1540821273966648019,finish=1540821332473027705,duration=58506379686
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:53:55] .................................................................................................... 2200/4977
[00:54:00] .................................................................................................... 2300/4977
[00:54:03] .................................................................................................... 2400/4977
[00:54:07] .................................................................................................... 2500/4977
[00:54:11] .............................................................iiiiiiiii.............................. 2600/4977
[00:54:19] ............ii...................................................................................... 2800/4977
[00:54:22] .................................................................................................... 2900/4977
[00:54:26] .................................................................................................... 3000/4977
[00:54:29] .......i............................................................................................ 3100/4977
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:17] 
[01:08:17] running 112 tests
[01:08:20] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:08:20] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:08:20] 
[01:08:20]  finished in 3.693
[01:08:20] travis_fold:end:test_codegen
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:45] 
[01:09:45] running 97 tests
[01:11:47] ..F....................................................F.test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:14:01] failures:
[01:14:01] 
[01:14:01] ---- [run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs stdout ----
[01:14:01] 
---
[01:14:01] 
[01:14:01] ------------------------------------------
[01:14:01] stderr:
[01:14:01] ------------------------------------------
[01:14:01] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Diagnostic { level: Fatal, message: [("parse error", NoStyle)], code: None, span: MultiSpan { primary_spans: [Span { lo: BytePos(8), hi: BytePos(11), ctxt: #0 }], span_labels: [] }, children: [], suggestions: [] }', libcore/result.rs:1009:5
[01:14:01] 
[01:14:01] ------------------------------------------
[01:14:01] 
[01:14:01] thread '[run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
---
[01:14:01] status: exit code: 101
[01:14:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/a"
[01:14:01] stdout:
[01:14:01] ------------------------------------------
[01:14:01] printed: box box x
[01:14:01] printed: box x()
[01:14:01] ------------------------------------------
[01:14:01] stderr:
[01:14:01] ------------------------------------------
[01:14:01] ------------------------------------------
[01:14:01] thread 'main' panicked at 'exprs are not equal:
[01:14:01]   e =      "(box ((x)()))"
[01:14:01]   parsed = "(box (box (x)))"', /checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs:223:9
[01:14:01] 
[01:14:01] ------------------------------------------
[01:14:01] 
[01:14:01] thread '[run-pass] run-pass-fulldeps/pprust-expr-roundtrip.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
---
[01:14:01] 
[01:14:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:14:01] 
[01:14:01] 
[01:14:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:01] 
[01:14:01] 
[01:14:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:01] Build completed unsuccessfully in 0:25:10
[01:14:01] Build completed unsuccessfully in 0:25:10
[01:14:01] make: *** [check] Error 1
[01:14:01] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1540c440
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:251f2778:start=1540825787914896244,finish=1540825787920399493,duration=5503249
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1463e26e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03f4f68e
travis_time:start:03f4f68e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15f92124
$ dmesg | grep -i kill
