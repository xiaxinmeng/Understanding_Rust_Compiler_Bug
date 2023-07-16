plain
travis_time:end:259b1a61:start=1556630128815116951,finish=1556630213828774618,duration=85013657667
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:08] .................................................................................................... 300/2958
[01:11:18] .................................................................................................... 400/2958
[01:11:27] .................................................................................................... 500/2958
[01:11:39] .................................................................................................... 600/2958
[01:11:54] ...........................................................................F........................ 700/2958
[01:12:14] .................................................................................................... 900/2958
[01:12:29] .................................................................................................... 1000/2958
[01:12:42] .................................................................................................... 1100/2958
[01:12:52] .................................................................................................... 1200/2958
---
[01:17:12] failures:
[01:17:12] 
[01:17:12] ---- [run-pass] run-pass/existential_type.rs stdout ----
[01:17:12] 
[01:17:12] error: test compilation failed although it shouldn't!
[01:17:12] status: exit code: 1
[01:17:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/existential_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/auxiliary"
[01:17:12] ------------------------------------------
[01:17:12] 
[01:17:12] ------------------------------------------
[01:17:12] stderr:
[01:17:12] stderr:
[01:17:12] ------------------------------------------
[01:17:12] error: at least one trait must be specified
[01:17:12]   --> /checkout/src/test/run-pass/existential_type.rs:71:46
[01:17:12]    |
[01:17:12] LL | existential type GenericBound<'a, T: Trait>: 'a;
[01:17:12] 
[01:17:12] error: at least one trait must be specified
[01:17:12]   --> /checkout/src/test/run-pass/existential_type.rs:78:42
[01:17:12]    |
[01:17:12]    |
[01:17:12] LL |     pub existential type Passthrough<T>: 'static;
[01:17:12] 
[01:17:12] error: aborting due to 2 previous errors
[01:17:12] 
[01:17:12] 
---
[01:17:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:17:12] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:12] 
[01:17:12] 
[01:17:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:12] 
[01:17:12] 
[01:17:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:12] Build completed unsuccessfully in 0:11:00
[01:17:12] Build completed unsuccessfully in 0:11:00
[01:17:12] make: *** [check] Error 1
[01:17:12] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01216b86
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 14:34:15 UTC 2019
---
travis_time:end:07208f76:start=1556634857210989565,finish=1556634857215890524,duration=4900959
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05f3d105
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b7b66f
travis_time:start:03b7b66f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a171200
$ dmesg | grep -i kill
