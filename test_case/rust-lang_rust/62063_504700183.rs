plain
travis_time:end:1a54d7b9:start=1561235720805642006,finish=1561235721533614591,duration=727972585
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:57:52] .................................................................................................... 500/2922
[00:58:03] .................................................................................................... 600/2922
[00:58:18] .................................................................................................... 700/2922
[00:58:29] .................................................................................................... 800/2922
[00:58:37] .............................................F...............F..F................................... 900/2922
[00:59:02] .................................................................................................... 1100/2922
[00:59:11] .................................................................................................... 1200/2922
[00:59:21] .................................................................................................... 1300/2922
[00:59:33] ......................ii............................................................................ 1400/2922
---
[01:03:17] failures:
[01:03:17] 
[01:03:17] ---- [run-pass] run-pass/generator/drop-env.rs stdout ----
[01:03:17] 
[01:03:17] error: test compilation failed although it shouldn't!
[01:03:17] status: exit code: 101
[01:03:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/drop-env.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/drop-env/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/drop-env/auxiliary"
[01:03:17] ------------------------------------------
[01:03:17] 
[01:03:17] ------------------------------------------
[01:03:17] stderr:
[01:03:17] stderr:
[01:03:17] ------------------------------------------
[01:03:17] thread 'rustc' panicked at 'Unreachable basic blocks during dataflow analysis', src/librustc_mir/util/liveness.rs:92:5
[01:03:17] 
[01:03:17] error: internal compiler error: unexpected panic
[01:03:17] 
[01:03:17] note: the compiler unexpectedly panicked. this is a bug.
[01:03:17] note: the compiler unexpectedly panicked. this is a bug.
[01:03:17] 
[01:03:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:03:17] 
[01:03:17] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:03:17] 
[01:03:17] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:03:17] 
[01:03:17] ------------------------------------------
[01:03:17] 
[01:03:17] 
[01:03:17] 
[01:03:17] ---- [run-pass] run-pass/generator/smoke.rs stdout ----
[01:03:17] 
[01:03:17] error: test compilation failed although it shouldn't!
[01:03:17] status: exit code: 101
[01:03:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/smoke.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/smoke/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/smoke/auxiliary"
[01:03:17] ------------------------------------------
[01:03:17] 
[01:03:17] ------------------------------------------
[01:03:17] stderr:
[01:03:17] stderr:
[01:03:17] ------------------------------------------
[01:03:17] thread 'rustc' panicked at 'Unreachable basic blocks during dataflow analysis', src/librustc_mir/util/liveness.rs:92:5
[01:03:17] 
[01:03:17] error: internal compiler error: unexpected panic
[01:03:17] 
[01:03:17] note: the compiler unexpectedly panicked. this is a bug.
[01:03:17] note: the compiler unexpectedly panicked. this is a bug.
[01:03:17] 
[01:03:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:03:17] 
[01:03:17] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:03:17] 
[01:03:17] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:03:17] 
[01:03:17] ------------------------------------------
[01:03:17] 
[01:03:17] 
[01:03:17] 
[01:03:17] ---- [run-pass] run-pass/generator/xcrate.rs stdout ----
[01:03:17] 
[01:03:17] error: auxiliary build of "/checkout/src/test/run-pass/generator/auxiliary/xcrate.rs" failed to compile: 
[01:03:17] status: exit code: 101
[01:03:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/auxiliary/xcrate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/xcrate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/xcrate/auxiliary"
[01:03:17] ------------------------------------------
[01:03:17] 
[01:03:17] ------------------------------------------
[01:03:17] stderr:
[01:03:17] stderr:
[01:03:17] ------------------------------------------
[01:03:17] warning: trait objects without an explicit `dyn` are deprecated
[01:03:17]   --> /checkout/src/test/run-pass/generator/auxiliary/xcrate.rs:14:37
[01:03:17]    |
[01:03:17] LL | pub fn bar<T: 'static>(t: T) -> Box<Generator<Yield = T, Return = ()> + Unpin> {
[01:03:17]    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Generator<Yield = T, Return = ()> + Unpin`
[01:03:17]    = note: #[warn(bare_trait_objects)] on by default
[01:03:17] 
[01:03:17] 
[01:03:17] thread 'rustc' panicked at 'Unreachable basic blocks during dataflow analysis', src/librustc_mir/util/liveness.rs:92:5
[01:03:17] 
[01:03:17] error: internal compiler error: unexpected panic
[01:03:17] 
[01:03:17] note: the compiler unexpectedly panicked. this is a bug.
[01:03:17] note: the compiler unexpectedly panicked. this is a bug.
[01:03:17] 
[01:03:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:03:17] 
[01:03:17] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:03:17] 
[01:03:17] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
[01:03:17] 
[01:03:17] ------------------------------------------
[01:03:17] 
[01:03:17] 
---
[01:03:17] test result: FAILED. 2910 passed; 3 failed; 9 ignored; 0 measured; 0 filtered out
[01:03:17] 
[01:03:17] 
[01:03:17] 
[01:03:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:17] 
[01:03:17] 
[01:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:17] Build completed unsuccessfully in 0:58:24
---
travis_time:end:2991ce3c:start=1561239531235601865,finish=1561239531240426254,duration=4824389
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f3dced4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!chec
