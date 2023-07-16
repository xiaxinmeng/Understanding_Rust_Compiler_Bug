plain
travis_time:end:077e1133:start=1561234594623482435,finish=1561234595391971367,duration=768488932
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:59:03] .................................................................................................... 500/2922
[00:59:14] .................................................................................................... 600/2922
[00:59:29] .................................................................................................... 700/2922
[00:59:40] .................................................................................................... 800/2922
[00:59:48] .............................................F...............F..F................................... 900/2922
[01:00:13] .................................................................................................... 1100/2922
[01:00:22] .................................................................................................... 1200/2922
[01:00:32] .................................................................................................... 1300/2922
[01:00:44] ......................ii............................................................................ 1400/2922
---
[01:04:33] failures:
[01:04:33] 
[01:04:33] ---- [run-pass] run-pass/generator/drop-env.rs stdout ----
[01:04:33] 
[01:04:33] error: test compilation failed although it shouldn't!
[01:04:33] status: exit code: 101
[01:04:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/drop-env.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/drop-env/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/drop-env/auxiliary"
[01:04:33] ------------------------------------------
[01:04:33] 
[01:04:33] ------------------------------------------
[01:04:33] stderr:
[01:04:33] stderr:
[01:04:33] ------------------------------------------
[01:04:33] thread 'rustc' panicked at 'Unreachable basic blocks during dataflow analysis', src/librustc_mir/dataflow/mod.rs:260:9
[01:04:33] 
[01:04:33] error: internal compiler error: unexpected panic
[01:04:33] 
[01:04:33] note: the compiler unexpectedly panicked. this is a bug.
[01:04:33] note: the compiler unexpectedly panicked. this is a bug.
[01:04:33] 
[01:04:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:33] 
[01:04:33] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:04:33] 
[01:04:33] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:04:33] 
[01:04:33] ------------------------------------------
[01:04:33] 
[01:04:33] 
[01:04:33] 
[01:04:33] ---- [run-pass] run-pass/generator/smoke.rs stdout ----
[01:04:33] 
[01:04:33] error: test compilation failed although it shouldn't!
[01:04:33] status: exit code: 101
[01:04:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/smoke.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/smoke/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/smoke/auxiliary"
[01:04:33] ------------------------------------------
[01:04:33] 
[01:04:33] ------------------------------------------
[01:04:33] stderr:
[01:04:33] stderr:
[01:04:33] ------------------------------------------
[01:04:33] thread 'rustc' panicked at 'Unreachable basic blocks during dataflow analysis', src/librustc_mir/dataflow/mod.rs:260:9
[01:04:33] 
[01:04:33] error: internal compiler error: unexpected panic
[01:04:33] 
[01:04:33] note: the compiler unexpectedly panicked. this is a bug.
[01:04:33] note: the compiler unexpectedly panicked. this is a bug.
[01:04:33] 
[01:04:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:33] 
[01:04:33] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:04:33] 
[01:04:33] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:04:33] 
[01:04:33] ------------------------------------------
[01:04:33] 
[01:04:33] 
[01:04:33] 
[01:04:33] ---- [run-pass] run-pass/generator/xcrate.rs stdout ----
[01:04:33] 
[01:04:33] error: auxiliary build of "/checkout/src/test/run-pass/generator/auxiliary/xcrate.rs" failed to compile: 
[01:04:33] status: exit code: 101
[01:04:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/auxiliary/xcrate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/xcrate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/xcrate/auxiliary"
[01:04:33] ------------------------------------------
[01:04:33] 
[01:04:33] ------------------------------------------
[01:04:33] stderr:
[01:04:33] stderr:
[01:04:33] ------------------------------------------
[01:04:33] warning: trait objects without an explicit `dyn` are deprecated
[01:04:33]   --> /checkout/src/test/run-pass/generator/auxiliary/xcrate.rs:14:37
[01:04:33]    |
[01:04:33] LL | pub fn bar<T: 'static>(t: T) -> Box<Generator<Yield = T, Return = ()> + Unpin> {
[01:04:33]    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Generator<Yield = T, Return = ()> + Unpin`
[01:04:33]    = note: #[warn(bare_trait_objects)] on by default
[01:04:33] 
[01:04:33] 
[01:04:33] thread 'rustc' panicked at 'Unreachable basic blocks during dataflow analysis', src/librustc_mir/dataflow/mod.rs:260:9
[01:04:33] 
[01:04:33] error: internal compiler error: unexpected panic
[01:04:33] 
[01:04:33] note: the compiler unexpectedly panicked. this is a bug.
[01:04:33] note: the compiler unexpectedly panicked. this is a bug.
[01:04:33] 
[01:04:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:33] 
[01:04:33] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:04:33] 
[01:04:33] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
[01:04:33] 
[01:04:33] ------------------------------------------
[01:04:33] 
[01:04:33] 
---
[01:04:33] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:04:33] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:33] 
[01:04:33] 
[01:04:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:33] 
[01:04:33] 
[01:04:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:33] Build completed unsuccessfully in 0:59:46
---
travis_time:end:0576da13:start=1561238480691749446,finish=1561238480696914280,duration=5164834
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09f123fc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19b91ea8
travis_time:start:19b91ea8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b6ca579
$ dmesg | grep -i kill
