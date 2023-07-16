plain
travis_time:end:0f92f179:start=1558866235610770029,finish=1558866324621797828,duration=89011027799
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:34] .................................................................................................... 2700/5589
[01:12:39] .................................................................................................... 2800/5589
[01:12:43] .................................................................................................... 2900/5589
[01:12:47] .................................................................................................... 3000/5589
[01:12:51] ..............F..................................................................................... 3100/5589
[01:12:58] .................................................................................................... 3300/5589
[01:13:02] ....................i............................................................................... 3400/5589
[01:13:05] ..............................................................................................ii...i 3500/5589
[01:13:09] ..ii................................................................................................ 3600/5589
---
[01:14:31] 
[01:14:31] ---- [ui] ui/issues/issue-60989.rs stdout ----
[01:14:31] diff of stderr:
[01:14:31] 
[01:14:31] + warning: trait objects without an explicit `dyn` are deprecated
[01:14:31] +    |
[01:14:31] +    |
[01:14:31] + LL |     c1::<Into<B>>;
[01:14:31] +    |          ^^^^^^^ help: use `dyn`: `dyn Into<B>`
[01:14:31] +    |
[01:14:31] +    = note: #[warn(bare_trait_objects)] on by default
[01:14:31] 1 error[E0109]: type arguments are not allowed for this type
[01:14:31] 2   --> $DIR/issue-60989.rs:12:10
[01:14:31] 3    |
[01:14:31] 
[01:14:31] 
[01:14:31] 
[01:14:31] The actual stderr differed from the expected stderr.
[01:14:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60989/issue-60989.stderr
[01:14:31] To update references, rerun the tests and pass the `--bless` flag
[01:14:31] To only update this specific test, also pass `--test-args issues/issue-60989.rs`
[01:14:31] error: 1 errors occurred comparing output.
[01:14:31] status: exit code: 1
[01:14:31] status: exit code: 1
[01:14:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60989.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60989" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60989/auxiliary" "-A" "unused"
[01:14:31] ------------------------------------------
[01:14:31] 
[01:14:31] ------------------------------------------
[01:14:31] stderr:
[01:14:31] stderr:
[01:14:31] ------------------------------------------
[01:14:31] warning: trait objects without an explicit `dyn` are deprecated
[01:14:31]    |
[01:14:31]    |
[01:14:31] LL |     c1::<Into<B>>;
[01:14:31]    |          ^^^^^^^ help: use `dyn`: `dyn Into<B>`
[01:14:31]    |
[01:14:31]    = note: #[warn(bare_trait_objects)] on by default
[01:14:31] error[E0109]: type arguments are not allowed for this type
[01:14:31]   --> /checkout/src/test/ui/issues/issue-60989.rs:12:10
[01:14:31]    |
[01:14:31] LL |     c1::<()>;
[01:14:31] LL |     c1::<()>;
[01:14:31]    |          ^^ type argument not allowed
[01:14:31] 
[01:14:31] error[E0109]: type arguments are not allowed for this type
[01:14:31]   --> /checkout/src/test/ui/issues/issue-60989.rs:16:10
[01:14:31]    |
[01:14:31] LL |     c1::<Into<B>>;
[01:14:31] 
[01:14:31] error: aborting due to 2 previous errors
[01:14:31] 
[01:14:31] For more information about this error, try `rustc --explain E0109`.
---
[01:14:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:14:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:31] 
[01:14:31] 
[01:14:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:31] 
[01:14:31] 
[01:14:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:31] Build completed unsuccessfully in 0:04:54
[01:14:31] Build completed unsuccessfully in 0:04:54
[01:14:31] Makefile:48: recipe for target 'check' failed
[01:14:31] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21a86d23
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 11:40:05 UTC 2019
---
travis_time:end:00be3c25:start=1558870806633055142,finish=1558870806638375133,duration=5319991
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:131ff56b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00c7d430
travis_time:start:00c7d430
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0334feb8
$ dmesg | grep -i kill
