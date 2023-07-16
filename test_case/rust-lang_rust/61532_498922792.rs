plain
travis_time:end:05d39f78:start=1559700871284598444,finish=1559700872064748223,duration=780149779
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:04] .................................................................................................... 400/5626
[00:55:07] .................................................................................................... 500/5626
[00:55:10] .i.................................................................................................. 600/5626
[00:55:14] .................................................................................................... 700/5626
[00:55:19] .........................................................................F.......................... 800/5626
[00:55:27] ......i............................................................................................. 1000/5626
[00:55:31] .......................iiiii........................................................................ 1100/5626
[00:55:34] .................................................................................................... 1200/5626
[00:55:36] .................................................................................................... 1300/5626
---
[00:58:18] 
[00:58:18] ---- [ui] ui/consts/const-eval/promoted_errors.rs stdout ----
[00:58:18] diff of stderr:
[00:58:18] 
[00:58:18] 16 LL |     println!("{}", 1/(1-1));
[00:58:18] 18 
[00:58:18] + warning: this expression will panic at runtime
[00:58:18] +   --> $DIR/promoted_errors.rs:9:20
[00:58:18] +    |
[00:58:18] +    |
[00:58:18] + LL |     println!("{}", 1/(1-1));
[00:58:18] + 
[00:58:18] 19 warning: attempt to divide by zero
[00:58:18] 20   --> $DIR/promoted_errors.rs:11:14
[00:58:18] 21    |
[00:58:18] 21    |
[00:58:18] 
[00:58:18] 33    |
[00:58:18] 34 LL |     println!("{}", 1/(false as u32));
[00:58:18] + 
[00:58:18] + warning: this expression will panic at runtime
[00:58:18] +   --> $DIR/promoted_errors.rs:14:20
[00:58:18] +    |
[00:58:18] +    |
[00:58:18] + LL |     println!("{}", 1/(false as u32));
[00:58:18] 36 
[00:58:18] 37 warning: attempt to divide by zero
[00:58:18] 38   --> $DIR/promoted_errors.rs:16:14
[00:58:18] 
[00:58:18] 
[00:58:18] 
[00:58:18] The actual stderr differed from the expected stderr.
[00:58:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/promoted_errors.stderr
[00:58:18] To update references, rerun the tests and pass the `--bless` flag
[00:58:18] To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors.rs`
[00:58:18] error: 1 errors occurred comparing output.
[00:58:18] status: exit code: 0
[00:58:18] status: exit code: 0
[00:58:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/auxiliary" "-A" "unused"
[00:58:18] ------------------------------------------
[00:58:18] 
[00:58:18] ------------------------------------------
[00:58:18] stderr:
---
[00:58:18] 
[00:58:18] warning: attempt to divide by zero
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:20
[00:58:18]    |
[00:58:18] LL |     println!("{}", 1/(1-1));
[00:58:18] 
[00:58:18] warning: this expression will panic at runtime
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:20
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     println!("{}", 1/(1-1));
[00:58:18] 
[00:58:18] warning: attempt to divide by zero
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:11:14
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     let _x = 1/(1-1);
[00:58:18] 
[00:58:18] warning: this expression will panic at runtime
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:11:14
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     let _x = 1/(1-1);
[00:58:18] 
[00:58:18] warning: attempt to divide by zero
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:14:20
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     println!("{}", 1/(false as u32));
[00:58:18] 
[00:58:18] warning: this expression will panic at runtime
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:14:20
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     println!("{}", 1/(false as u32));
[00:58:18] 
[00:58:18] warning: attempt to divide by zero
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:16:14
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     let _x = 1/(false as u32);
[00:58:18] 
[00:58:18] warning: this expression will panic at runtime
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:16:14
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     let _x = 1/(false as u32);
[00:58:18] 
[00:58:18] warning: reaching this expression at runtime will panic or abort
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:14:20
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     println!("{}", 1/(false as u32));
[00:58:18] 
[00:58:18] warning: reaching this expression at runtime will panic or abort
[00:58:18]   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:20
[00:58:18]    |
[00:58:18]    |
[00:58:18] LL |     println!("{}", 1/(1-1));
[00:58:18] 
[00:58:18] 
[00:58:18] ------------------------------------------
[00:58:18] 
---
[00:58:18] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:58:18] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:18] 
[00:58:18] 
[00:58:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:18] 
[00:58:18] 
[00:58:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:18] Build completed unsuccessfully in 0:53:42
---
travis_time:end:1a556f54:start=1559704382739029581,finish=1559704382743666833,duration=4637252
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d36d960
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0115dbec
travis_time:start:0115dbec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cde0425
$ dmesg | grep -i kill
