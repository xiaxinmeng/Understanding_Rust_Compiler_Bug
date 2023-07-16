plain
travis_time:end:002d98da:start=1561144818851324805,finish=1561144821354389366,duration=2503064561
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:57:08] ...........................................................................ii...i..ii............... 3600/5695
[00:57:12] .................................................................................................... 3700/5695
[00:57:15] .................................................................................................... 3800/5695
[00:57:19] .....................................................................................ii............. 3900/5695
[00:57:22] .............................................FF..................................................... 4000/5695
[00:57:26] .........................................................................i.......................... 4200/5695
[00:57:28] .................................................................................................... 4300/5695
[00:57:35] .................................................................................................... 4400/5695
[00:57:45] .................................................................................................... 4500/5695
---
[00:58:30] failures:
[00:58:30] 
[00:58:30] ---- [ui] ui/or-patterns/basic-switch.rs stdout ----
[00:58:30] 
[00:58:30] error: /checkout/src/test/ui/or-patterns/basic-switch.rs:5: unexpected warning: '5:12: 5:23: the feature `or_patterns` is incomplete and may cause the compiler to crash'
[00:58:30] error: 1 unexpected errors found, 0 expected errors not found
[00:58:30] status: exit code: 101
[00:58:30] status: exit code: 101
[00:58:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/basic-switch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/basic-switch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/basic-switch/auxiliary" "-A" "unused"
[00:58:30]     Error {
[00:58:30]         line_num: 5,
[00:58:30]         kind: Some(
[00:58:30]             Warning,
[00:58:30]             Warning,
[00:58:30]         ),
[00:58:30]         msg: "5:12: 5:23: the feature `or_patterns` is incomplete and may cause the compiler to crash",
[00:58:30] ]
[00:58:30] 
[00:58:30] thread '[ui] ui/or-patterns/basic-switch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1499:13
[00:58:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:30] 
[00:58:30] ---- [ui] ui/or-patterns/mix-with-wild.rs stdout ----
[00:58:30] 
[00:58:30] error: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:5: unexpected warning: '5:12: 5:23: the feature `or_patterns` is incomplete and may cause the compiler to crash'
[00:58:30] 
[00:58:30] error: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10: unexpected error: '10:14: 10:19: src/librustc_mir/build/matches/test.rs:955: simplifyable pattern found: Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:19, kind: Or { pats: [Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:15, kind: Constant { value: Const { ty: usize, val: Scalar(0x0000000000000000) } } }, Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:18: 10:19, kind: Wild }] } }'
[00:58:30] error: 2 unexpected errors found, 0 expected errors not found
[00:58:30] status: exit code: 101
[00:58:30] status: exit code: 101
[00:58:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/mix-with-wild.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/mix-with-wild" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/mix-with-wild/auxiliary" "-A" "unused"
[00:58:30]     Error {
[00:58:30]         line_num: 5,
[00:58:30]         kind: Some(
[00:58:30]             Warning,
[00:58:30]             Warning,
[00:58:30]         ),
[00:58:30]         msg: "5:12: 5:23: the feature `or_patterns` is incomplete and may cause the compiler to crash",
[00:58:30]     Error {
[00:58:30]         line_num: 10,
[00:58:30]         kind: Some(
[00:58:30]             Error,
[00:58:30]             Error,
[00:58:30]         ),
[00:58:30]         msg: "10:14: 10:19: src/librustc_mir/build/matches/test.rs:955: simplifyable pattern found: Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:19, kind: Or { pats: [Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:15, kind: Constant { value: Const { ty: usize, val: Scalar(0x0000000000000000) } } }, Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:18: 10:19, kind: Wild }] } }",
[00:58:30] ]
[00:58:30] 
[00:58:30] 
[00:58:30] thread '[ui] ui/or-patterns/mix-with-wild.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1499:13
[00:58:30] 
[00:58:30] failures:
[00:58:30]     [ui] ui/or-patterns/basic-switch.rs
[00:58:30]     [ui] ui/or-patterns/basic-switch.rs
[00:58:30]     [ui] ui/or-patterns/mix-with-wild.rs
[00:58:30] test result: FAILED. 5672 passed; 2 failed; 21 ignored; 0 measured; 0 filtered out
[00:58:30] 
[00:58:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:58:30] 
[00:58:30] 
[00:58:30] 
[00:58:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:30] 
[00:58:30] 
[00:58:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:30] Build completed unsuccessfully in 0:53:48
---
travis_time:end:0e4080fd:start=1561148344536330176,finish=1561148344540834273,duration=4504097
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ae5c903
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:073484cc
travis_time:start:073484cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00cac0a4
$ dmesg | grep -i kill
