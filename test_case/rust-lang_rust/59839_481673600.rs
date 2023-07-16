plain
travis_time:end:046ca9e6:start=1554895634719746745,finish=1554895636999312526,duration=2279565781
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:06:33] .................................................................................................... 500/2961
[01:06:45] .................................................................................................... 600/2961
[01:07:01] .................................................................................................... 700/2961
[01:07:12] .................................................................................................... 800/2961
[01:07:21] ................F................................................................................... 900/2961
[01:07:49] .................................................................................................... 1100/2961
[01:07:58] .................................................................................................... 1200/2961
[01:08:08] .................................................................................................... 1300/2961
[01:08:20] .................................................................................................... 1400/2961
---
[01:12:14] failures:
[01:12:14] 
[01:12:14] ---- [run-pass] run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers.rs stdout ----
[01:12:14] normalized stderr:
[01:12:14] warning: unused return value of `core::num::<impl u32>::wrapping_add` that must be used
[01:12:14]    |
[01:12:14]    |
[01:12:14] LL |         y.wrapping_add(1);
[01:12:14]    |
[01:12:14]    = note: #[warn(unused_must_use)] on by default
[01:12:14]    = note: this returns the result of the operation, without modifying the original
[01:12:14] 
[01:12:14] 
[01:12:14] 
[01:12:14] 
[01:12:14] 
[01:12:14] The actual stderr differed from the expected stderr.
[01:12:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers/expect-infer-supply-two-infers.stderr
[01:12:14] To update references, rerun the tests and pass the `--bless` flag
[01:12:14] To only update this specific test, also pass `--test-args functions-closures/closure-expected-type/expect-infer-supply-two-infers.rs`
[01:12:14] error: 1 errors occurred comparing output.
[01:12:14] status: exit code: 0
[01:12:14] status: exit code: 0
[01:12:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers/auxiliary"
[01:12:14] ------------------------------------------
[01:12:14] 
[01:12:14] ------------------------------------------
[01:12:14] stderr:
[01:12:14] stderr:
[01:12:14] ------------------------------------------
[01:12:14] {"message":"unused return value of `core::num::<impl u32>::wrapping_add` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers.rs","byte_start":397,"byte_end":415,"line_start":15,"line_end":15,"column_start":9,"column_end":27,"is_primary":true,"text":[{"text":"        y.wrapping_add(1);","highlight_start":9,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this returns the result of the operation, without modifying the original","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused return value of `core::num::<impl u32>::wrapping_add` that must be used\n  --> /checkout/src/test/run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers.rs:15:9\n   |\nLL |         y.wrapping_add(1);\n   |         ^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n   = note: this returns the result of the operation, without modifying the original\n\n"}
[01:12:14] ------------------------------------------
[01:12:14] 
[01:12:14] thread '[run-pass] run-pass/functions-closures/closure-expected-type/expect-infer-supply-two-infers.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:12:14] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:12:14] 
[01:12:14] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:12:14] 
[01:12:14] 
[01:12:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:14] 
[01:12:14] 
[01:12:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:14] Build completed unsuccessfully in 0:10:54
[01:12:14] Build completed unsuccessfully in 0:10:54
[01:12:14] make: *** [check] Error 1
[01:12:14] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0840f0f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 12:39:42 UTC 2019
