plain
travis_time:end:19a3dde7:start=1556874160983640778,finish=1556874161888929834,duration=905289056
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:22] .................................................................................................... 300/5483
[01:09:26] .................................................................................................... 400/5483
[01:09:29] .............................................................................................i...... 500/5483
[01:09:33] .................................................................................................... 600/5483
[01:09:36] .............................................................F...................................... 700/5483
[01:09:47] .................................................................i...............i.................. 900/5483
[01:09:50] ..................................................................................................ii 1000/5483
[01:09:55] iii................................................................................................. 1100/5483
[01:09:57] .................................................................................................... 1200/5483
---
[01:12:38] diff of stderr:
[01:12:38] 
[01:12:38] 8   --> $DIR/cannot-infer-const-args.rs:9:5
[01:12:38] 9    |
[01:12:38] 10 LL |     foo();
[01:12:38] -    |     ^^^ cannot infer type for `fn() -> usize {foo::<_>}`
[01:12:38] +    |     ^^^ cannot infer type for `fn() -> usize {foo::<_: usize>}`
[01:12:38] 13 error: aborting due to previous error
[01:12:38] 14 
[01:12:38] 
[01:12:38] 
[01:12:38] 
[01:12:38] The actual stderr differed from the expected stderr.
[01:12:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-const-args/cannot-infer-const-args.stderr
[01:12:38] To update references, rerun the tests and pass the `--bless` flag
[01:12:38] To only update this specific test, also pass `--test-args const-generics/cannot-infer-const-args.rs`
[01:12:38] error: 1 errors occurred comparing output.
[01:12:38] status: exit code: 1
[01:12:38] status: exit code: 1
[01:12:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/cannot-infer-const-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-const-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-const-args/auxiliary" "-A" "unused"
[01:12:38] ------------------------------------------
[01:12:38] 
[01:12:38] ------------------------------------------
[01:12:38] stderr:
---
[01:12:38] 
[01:12:38] error[E0282]: type annotations needed
[01:12:38]   --> /checkout/src/test/ui/const-generics/cannot-infer-const-args.rs:9:5
[01:12:38]    |
[01:12:38] LL |     foo(); //~ ERROR type annotations needed
[01:12:38]    |     ^^^ cannot infer type for `fn() -> usize {foo::<_: usize>}`
[01:12:38] error: aborting due to previous error
[01:12:38] 
[01:12:38] For more information about this error, try `rustc --explain E0282`.
[01:12:38] 
---
[01:12:38] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:12:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:38] 
[01:12:38] 
[01:12:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:38] 
[01:12:38] 
[01:12:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:38] Build completed unsuccessfully in 0:04:25
[01:12:38] Build completed unsuccessfully in 0:04:25
[01:12:38] make: *** [check] Error 1
[01:12:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1994f100
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May  3 10:15:31 UTC 2019
---
travis_time:end:1a2d8764:start=1556878533061334374,finish=1556878533065916123,duration=4581749
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f067bd0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13b308ee
travis_time:start:13b308ee
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08bd73f9
$ dmesg | grep -i kill
