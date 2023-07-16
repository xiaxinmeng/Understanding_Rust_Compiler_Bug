plain
travis_time:end:273f9690:start=1556627412079245594,finish=1556627416078293462,duration=3999047868
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:14:57] .................................................................................................... 300/2958
[01:15:09] .................................................................................................... 400/2958
[01:15:18] .................................................................................................... 500/2958
[01:15:30] .................................................................................................... 600/2958
[01:15:45] ..........................................................................F......................... 700/2958
[01:16:06] .................................................................................................... 900/2958
[01:16:22] .................................................................................................... 1000/2958
[01:16:36] .................................................................................................... 1100/2958
[01:16:46] .................................................................................................... 1200/2958
---
[01:21:14] failures:
[01:21:14] 
[01:21:14] ---- [run-pass] run-pass/existential_type.rs stdout ----
[01:21:14] 
[01:21:14] error: test compilation failed although it shouldn't!
[01:21:14] status: exit code: 1
[01:21:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/existential_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/auxiliary"
[01:21:14] ------------------------------------------
[01:21:14] 
[01:21:14] ------------------------------------------
[01:21:14] stderr:
[01:21:14] stderr:
[01:21:14] ------------------------------------------
[01:21:14] error: at least one trait must be specified
[01:21:14]   --> /checkout/src/test/run-pass/existential_type.rs:71:46
[01:21:14]    |
[01:21:14] LL | existential type GenericBound<'a, T: Trait>: 'a;
[01:21:14] 
[01:21:14] error: at least one trait must be specified
[01:21:14]   --> /checkout/src/test/run-pass/existential_type.rs:78:42
[01:21:14]    |
[01:21:14]    |
[01:21:14] LL |     pub existential type Passthrough<T>: 'static;
[01:21:14] 
[01:21:14] error: aborting due to 2 previous errors
[01:21:14] 
[01:21:14] 
---
[01:21:14] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:21:14] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:21:14] 
[01:21:14] 
[01:21:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:14] 
[01:21:14] 
[01:21:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:14] Build completed unsuccessfully in 0:11:27
[01:21:14] Build completed unsuccessfully in 0:11:27
[01:21:14] Makefile:48: recipe for target 'check' failed
[01:21:14] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15066535
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 13:51:39 UTC 2019
---
travis_time:end:0365b980:start=1556632301362664943,finish=1556632301367669463,duration=5004520
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ac76e6a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
