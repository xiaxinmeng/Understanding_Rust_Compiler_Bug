plain
travis_time:end:0ec86515:start=1560367100514842700,finish=1560367101292668785,duration=777826085
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:54:47] .................................................................................................... 400/5677
[00:54:51] .................................................................................................... 500/5677
[00:54:54] ...................................i................................................................ 600/5677
[00:54:58] .................................................................................................... 700/5677
[00:55:02] ...........................................F........................................................ 800/5677
[00:55:11] ......................................i...........i................................................. 1000/5677
[00:55:14] ...................................................................iiiii............................ 1100/5677
[00:55:18] .................................................................................................... 1200/5677
[00:55:20] .................................................................................................... 1300/5677
---
[00:58:02] 8 
[00:58:02] + error: this expression will panic at runtime
[00:58:02] +   --> $DIR/array-literal-index-oob.rs:2:5
[00:58:02] +    |
[00:58:02] + LL |     &{[1, 2, 3][4]};
[00:58:02] +    |     ^^^^^^^^^^^^^^^ index out of bounds: the len is 3 but the index is 4
[00:58:02] 9 error: reaching this expression at runtime will panic or abort
[00:58:02] 10   --> $DIR/array-literal-index-oob.rs:2:7
[00:58:02] 11    |
[00:58:02] 
---
[00:58:02] 19 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/array-literal-index-oob.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args consts/array-literal-index-oob.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/array-literal-index-oob.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: index out of bounds: the len is 3 but the index is 4
[00:58:02]   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:2:7
[00:58:02]    |
[00:58:02] LL |     &{[1, 2, 3][4]};
[00:58:02]    |
[00:58:02]    = note: #[deny(const_err)] on by default
[00:58:02] 
[00:58:02] error: this expression will panic at runtime
[00:58:02] error: this expression will panic at runtime
[00:58:02]   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:2:5
[00:58:02]    |
[00:58:02] LL |     &{[1, 2, 3][4]};
[00:58:02]    |     ^^^^^^^^^^^^^^^ index out of bounds: the len is 3 but the index is 4
[00:58:02] error: reaching this expression at runtime will panic or abort
[00:58:02]   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:2:7
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     &{[1, 2, 3][4]};
[00:58:02]    |     --^^^^^^^^^^^^-
[00:58:02]    |       index out of bounds: the len is 3 but the index is 4
[00:58:02] 
[00:58:02] error: aborting due to 3 previous errors
[00:58:02] 
---
[00:58:02] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:58:02] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:02] 
[00:58:02] 
[00:58:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:02] 
[00:58:02] 
[00:58:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:02] Build completed unsuccessfully in 0:53:26
---
travis_time:end:245455ad:start=1560370596575203183,finish=1560370596579811930,duration=4608747
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:042f6dc8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:036f7f18
travis_time:start:036f7f18
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0325a57a
$ dmesg | grep -i kill
