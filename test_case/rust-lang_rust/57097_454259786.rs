plain
travis_time:end:32e229d0:start=1547520899715181293,finish=1547520973691669871,duration=73976488578
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:25] .................................................................................................... 500/5302
[01:02:28] .............................i...................................................................... 600/5302
[01:02:32] .................................................................................................... 700/5302
[01:02:37] .................................................................................................... 800/5302
[01:02:43] .....................................................................i...............i...F.F.FFFFFF. 900/5302
[01:02:46] ...............................................................................................iiiii 1000/5302
[01:02:52] .................................................................................................... 1200/5302
[01:02:55] .................................................................................................... 1300/5302
[01:02:58] .................................................................................................... 1400/5302
[01:03:00] .................................................................................................... 1500/5302
---
[01:05:20] failures:
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:20] 
[01:05:20] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[01:05:20] 
[01:05:20] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:20] status: exit code: 101
[01:05:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] stderr:
---
[01:05:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:20] 
[01:05:20] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:05:20] 
[01:05:20] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:05:20] 
[01:05:20] ------------------------------------------
[01:05:20] 
[01:05:20] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:05:20] 
[01:05:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:05:20] 
[01:05:20] 
[01:05:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:20] 
[01:05:20] 
[01:05:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:20] Build completed unsuccessfully in 0:04:11
[01:05:20] Build completed unsuccessfully in 0:04:11
[01:05:20] make: *** [check] Error 1
[01:05:20] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:148c4847
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 15 04:01:43 UTC 2019
---
travis_time:end:1c22cf64:start=1547524905024246486,finish=1547524905028908520,duration=4662034
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02aaa09c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$travis_time:end:026ae44e:start=1547524905044437752,finish=1547524905048590417,duration=4152665
travis_fold:start:after_failure.6
travis_time:start:03704d30
$ dmesg | grep -i kill
travis_time:end:03704d30:start=1547524905052725001,finish=1547524905062947567,duration=10222566
