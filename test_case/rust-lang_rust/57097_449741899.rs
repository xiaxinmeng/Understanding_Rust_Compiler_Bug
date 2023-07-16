plain
travis_time:end:11fd254a:start=1545659656922431285,finish=1545659657887977522,duration=965546237
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:46:04] .................................................................................................... 500/5199
[00:46:08] ..............................i..................................................................... 600/5199
[00:46:11] .................................................................................................... 700/5199
[00:46:16] .................................................................................................... 800/5199
[00:46:20] ..........i...............i...F..FFFFFFF............................................................ 900/5199
[00:46:23] ....................................iiiii........................................................... 1000/5199
[00:46:28] .................................................................................................... 1200/5199
[00:46:30] .................................................................................................... 1300/5199
[00:46:33] .................................................................................................... 1400/5199
[00:46:35] .................................................................................................... 1500/5199
---
[00:48:38] failures:
[00:48:38] 
[00:48:38] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[00:48:38] 
[00:48:38] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "--target=x86_64-unknown-linux-gnu" "--error-form---------
[00:48:38] stderr:
[00:48:38] ------------------------------------------
[00:48:38] ------------------------------------------
[00:48:38] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:381:13
[00:48:38] 
[00:48:38] error: internal compiler error: unexpected panic
[00:48:38] 
[00:48:38] note: the compiler unexpectedly panicked. this is a bug.
[00:48:38] note: the compiler unexpectedly panicked. this is a bug.
[00:48:38] 
[00:48:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:38] 
[00:48:38] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:48:38] 
[00:48:38] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:48:38] 
[00:48:38] ------------------------------------------
[00:48:38] 
[00:48:38] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:48:38] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:48:38] 
[00:48:38] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[00:48:38] 
[00:48:38] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:38] status: exit code: 101
[00:48:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[00:48:38] ------------------------------------------
[00:48:38] 
[00:48:38] ------------------------------------------
[00:48:38] stderr:
[00:48:38] stderr:
[00:48:38] ------------------------------------------
[00:48:38] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:381:13
[00:48:38] 
[00:48:38] error: internal compiler error: unexpected panic
[00:48:38] 
[00:48:38] note: the compiler unexpectedly panicked. this is a bug.
[00:48:38] note: the compiler unexpectedly panicked. this is a bug.
[00:48:38] 
[00:48:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:38] 
[00:48:38] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:48:38] 
[00:48:38] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:48:38] 
[00:48:38] ------------------------------------------
[00:48:38] 
[00:48:38] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:48:38] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:48:38] 
[00:48:38] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[00:48:38] 
[00:48:38] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:38] status: exit code: 101
[00:48:38] command: "/checkout/obj/build/x-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:38] 
[00:48:38] 
[00:48:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:38] Build completed unsuccessfully in 0:03:42
[00:48:38] Build completed unsuccessfully in 0:03:42
[00:48:38] Makefile:58: recipe for target 'check' failed
[00:48:38] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01e6566a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 24 14:43:04 UTC 2018
