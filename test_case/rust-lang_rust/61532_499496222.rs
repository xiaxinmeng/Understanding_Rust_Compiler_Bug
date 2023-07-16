plain
travis_time:end:04c19f40:start=1559824493424994042,finish=1559824494238364180,duration=813370138
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:00:06] 
[01:00:06] ---- [ui] ui/consts/const-eval/promoted_errors.rs stdout ----
[01:00:06] diff of stderr:
[01:00:06] 
[01:00:06] 16 LL |     println!("{}", 1/(1-1));
[01:00:06] 18 
[01:00:06] + warning: this expression will panic at runtime
[01:00:06] +   --> $DIR/promoted_errors.rs:9:20
[01:00:06] +    |
[01:00:06] +    |
[01:00:06] + LL |     println!("{}", 1/(1-1));
[01:00:06] + 
[01:00:06] 19 warning: attempt to divide by zero
[01:00:06] 20   --> $DIR/promoted_errors.rs:11:14
[01:00:06] 21    |
[01:00:06] 21    |
[01:00:06] 
[01:00:06] 33    |
[01:00:06] 34 LL |     println!("{}", 1/(false as u32));
[01:00:06] + 
[01:00:06] + warning: this expression will panic at runtime
[01:00:06] +   --> $DIR/promoted_errors.rs:14:20
[01:00:06] +    |
[01:00:06] +    |
[01:00:06] + LL |     println!("{}", 1/(false as u32));
[01:00:06] 36 
[01:00:06] 37 warning: attempt to divide by zero
[01:00:06] 38   --> $DIR/promoted_errors.rs:16:14
[01:00:06] 
[01:00:06] 
[01:00:06] 
[01:00:06] The actual stderr differed from the expected stderr.
[01:00:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/promoted_errors.stderr
[01:00:06] To update references, rerun the tests and pass the `--bless` flag
[01:00:06] To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors.rs`
[01:00:06] error: 1 errors occurred comparing output.
[01:00:06] status: exit code: 0
[01:00:06] status: exit code: 0
[01:00:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/auxiliary" "-A" "unused"
[01:00:06] ------------------------------------------
[01:00:06] 
[01:00:06] ------------------------------------------
[01:00:06] stderr:
---
[01:00:06] 
[01:00:06] warning: attempt to divide by zero
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:20
[01:00:06]    |
[01:00:06] LL |     println!("{}", 1/(1-1));
[01:00:06] 
[01:00:06] warning: this expression will panic at runtime
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:20
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     println!("{}", 1/(1-1));
[01:00:06] 
[01:00:06] warning: attempt to divide by zero
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:11:14
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     let _x = 1/(1-1);
[01:00:06] 
[01:00:06] warning: this expression will panic at runtime
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:11:14
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     let _x = 1/(1-1);
[01:00:06] 
[01:00:06] warning: attempt to divide by zero
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:14:20
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     println!("{}", 1/(false as u32));
[01:00:06] 
[01:00:06] warning: this expression will panic at runtime
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:14:20
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     println!("{}", 1/(false as u32));
[01:00:06] 
[01:00:06] warning: attempt to divide by zero
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:16:14
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     let _x = 1/(false as u32);
[01:00:06] 
[01:00:06] warning: this expression will panic at runtime
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:16:14
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     let _x = 1/(false as u32);
[01:00:06] 
[01:00:06] warning: reaching this expression at runtime will panic or abort
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:14:20
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     println!("{}", 1/(false as u32));
[01:00:06] 
[01:00:06] warning: reaching this expression at runtime will panic or abort
[01:00:06]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:20
[01:00:06]    |
[01:00:06]    |
[01:00:06] LL |     println!("{}", 1/(1-1));
[01:00:06] 
[01:00:06] 
[01:00:06] ------------------------------------------
[01:00:06] 
---
[01:00:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:00:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:00:06] 
[01:00:06] 
[01:00:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:06] 
[01:00:06] 
[01:00:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:06] Build completed unsuccessfully in 0:55:16
---
travis_time:end:0b168085:start=1559828124888391600,finish=1559828124895270240,duration=6878640
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aa476c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06026370
$ dmesg | grep -i kill
