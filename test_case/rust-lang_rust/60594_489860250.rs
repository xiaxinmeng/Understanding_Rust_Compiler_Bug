plain
travis_time:end:177e2346:start=1557184614876483789,finish=1557184615630420973,duration=753937184
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:17] .................................................................................................... 4500/5495
[01:13:20] .................................................................................................... 4600/5495
[01:13:25] .................................................................................................... 4700/5495
[01:13:31] .................................................................................................... 4800/5495
[01:13:34] ........................................F........................................................... 4900/5495
[01:13:42] .................................................................................................... 5100/5495
[01:13:46] .................................................................................................... 5200/5495
[01:13:50] .................................................................................................... 5300/5495
[01:13:53] .................................................................................................... 5400/5495
[01:13:53] .................................................................................................... 5400/5495
[01:13:56] .................................i.............................................................
[01:13:56] failures:
[01:13:56] 
[01:13:56] ---- [ui] ui/span/issue-29106.rs stdout ----
[01:13:56] diff of stderr:
[01:13:56] 
[01:13:56] 1 error[E0597]: `x` does not live long enough
[01:13:56] -    |
[01:13:56] -    |
[01:13:56] - LL |         y = Arc::new(Foo(&x));
[01:13:56] -    |                          ^^ borrowed value does not live long enough
[01:13:56] - LL |     }
[01:13:56] -    |     |
[01:13:56] -    |     |
[01:13:56] -    |     `x` dropped here while still borrowed
[01:13:56] -    |     borrow might be used here, when `y` is dropped and runs the `Drop` code for type `std::sync::Arc`
[01:13:56] -    |
[01:13:56] -    = note: values in a scope are dropped in the opposite order they are defined
[01:13:56] - 
[01:13:56] - error[E0597]: `x` does not live long enough
[01:13:56] 16    |
[01:13:56] 16    |
[01:13:56] 17 LL |         y = Rc::new(Foo(&x));
[01:13:56] 24    |
[01:13:56] 24    |
[01:13:56] 25    = note: values in a scope are dropped in the opposite order they are defined
[01:13:56] - error: aborting due to 2 previous errors
[01:13:56] + error: aborting due to previous error
[01:13:56] 28 
[01:13:56] 29 For more information about this error, try `rustc --explain E0597`.
[01:13:56] 29 For more information about this error, try `rustc --explain E0597`.
[01:13:56] 30 
[01:13:56] 
[01:13:56] 
[01:13:56] The actual stderr differed from the expected stderr.
[01:13:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-29106/issue-29106.stderr
[01:13:56] To update references, rerun the tests and pass the `--bless` flag
[01:13:56] To only update this specific test, also pass `--test-args span/issue-29106.rs`
[01:13:56] error: 1 errors occurred comparing output.
[01:13:56] status: exit code: 1
[01:13:56] status: exit code: 1
[01:13:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-29106.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-29106/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-29106/auxiliary" "-A" "unused"
[01:13:56] ------------------------------------------
[01:13:56] 
[01:13:56] ------------------------------------------
[01:13:56] stderr:
[01:13:56] stderr:
[01:13:56] ------------------------------------------
[01:13:56] error[E0597]: `x` does not live long enough
[01:13:56]    |
[01:13:56]    |
[01:13:56] LL |         y = Rc::new(Foo(&x));
[01:13:56]    |                         ^^ borrowed value does not live long enough
[01:13:56]    |     -
[01:13:56]    |     |
[01:13:56]    |     |
[01:13:56]    |     `x` dropped here while still borrowed
[01:13:56]    |     borrow might be used here, when `y` is dropped and runs the `Drop` code for type `std::rc::Rc`
[01:13:56]    |
[01:13:56]    = note: values in a scope are dropped in the opposite order they are defined
[01:13:56] error: aborting due to previous error
[01:13:56] 
[01:13:56] For more information about this error, try `rustc --explain E0597`.
[01:13:56] 
---
[01:13:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:13:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:56] 
[01:13:56] 
[01:13:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:56] 
[01:13:56] 
[01:13:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:56] Build completed unsuccessfully in 0:04:36
[01:13:56] Build completed unsuccessfully in 0:04:36
[01:13:56] Makefile:48: recipe for target 'check' failed
[01:13:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0207160f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May  7 00:31:03 UTC 2019
---
travis_time:end:00e7ed5c:start=1557189064519617056,finish=1557189064526121199,duration=6504143
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01e1a350
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04ba54d4
travis_time:start:04ba54d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07654d28
$ dmesg | grep -i kill
