plain
travis_time:end:00a53ce1:start=1561237324609047038,finish=1561237414417229609,duration=89808182571
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:53] .................................................................................................... 300/5568
[01:13:57] .................................................................................................... 400/5568
[01:14:00] ................................................................................................i... 500/5568
[01:14:03] .................................................................................................... 600/5568
[01:14:07] .......................................................................................F............ 700/5568
[01:14:17] ..............................................................................i...............i..... 900/5568
[01:14:21] .................................................................................................... 1000/5568
[01:14:24] ...........iiiii.................................................................................... 1100/5568
[01:14:27] .................................................................................................... 1200/5568
---
[01:17:07] 8 
[01:17:07] - error: this expression will panic at runtime
[01:17:07] -   --> $DIR/array-literal-index-oob.rs:2:5
[01:17:07] -    |
[01:17:07] - LL |     &{[1, 2, 3][4]};
[01:17:07] -    |     ^^^^^^^^^^^^^^^ index out of bounds: the len is 3 but the index is 4
[01:17:07] 15 error: reaching this expression at runtime will panic or abort
[01:17:07] 16   --> $DIR/array-literal-index-oob.rs:2:7
[01:17:07] 17    |
[01:17:07] 
---
[01:17:07] 25 
[01:17:07] 
[01:17:07] 
[01:17:07] The actual stderr differed from the expected stderr.
[01:17:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/array-literal-index-oob.stderr
[01:17:07] To update references, rerun the tests and pass the `--bless` flag
[01:17:07] To only update this specific test, also pass `--test-args consts/array-literal-index-oob.rs`
[01:17:07] error: 1 errors occurred comparing output.
[01:17:07] status: exit code: 1
[01:17:07] status: exit code: 1
[01:17:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/array-literal-index-oob.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/auxiliary" "-A" "unused"
[01:17:07] ------------------------------------------
[01:17:07] 
[01:17:07] ------------------------------------------
[01:17:07] stderr:
[01:17:07] stderr:
[01:17:07] ------------------------------------------
[01:17:07] error: index out of bounds: the len is 3 but the index is 4
[01:17:07]   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:2:7
[01:17:07]    |
[01:17:07] LL |     &{[1, 2, 3][4]};
[01:17:07]    |
[01:17:07]    = note: #[deny(const_err)] on by default
[01:17:07] 
[01:17:07] error: reaching this expression at runtime will panic or abort
[01:17:07] error: reaching this expression at runtime will panic or abort
[01:17:07]   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:2:7
[01:17:07]    |
[01:17:07] LL |     &{[1, 2, 3][4]};
[01:17:07]    |     --^^^^^^^^^^^^-
[01:17:07]    |       index out of bounds: the len is 3 but the index is 4
[01:17:07] 
[01:17:07] error: aborting due to 2 previous errors
[01:17:07] 
---
[01:17:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:17:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:07] 
[01:17:07] 
[01:17:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:07] 
[01:17:07] 
[01:17:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:07] Build completed unsuccessfully in 0:04:39
[01:17:07] Build completed unsuccessfully in 0:04:39
[01:17:07] make: *** [check] Error 1
[01:17:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00432f38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jun 22 22:20:51 UTC 2019
---
travis_time:end:0c9290b8:start=1561242052725631478,finish=1561242052730397658,duration=4766180
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01283998
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d864c6a
travis_time:start:1d864c6a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13b6f6ef
$ dmesg | grep -i kill
