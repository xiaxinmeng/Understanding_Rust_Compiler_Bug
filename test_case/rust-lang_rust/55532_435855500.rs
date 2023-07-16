plain
travis_time:end:03254a7e:start=1541417438953379725,finish=1541417441090165198,duration=2136785473
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:11] .................................................................................................... 2600/4989
[00:49:15] .................................................................................................... 2700/4989
[00:49:18] .................................................................................................... 2800/4989
[00:49:21] .................................................................................................... 2900/4989
[00:49:25] ..........................F......................................................................... 3000/4989
[00:49:31] ..............................................................................i.i..ii............... 3200/4989
[00:49:35] .................................................................................................... 3300/4989
[00:49:39] .................................................................................................... 3400/4989
[00:49:42] ...................................................i.ii............................................. 3500/4989
---
[00:50:27] 
[00:50:27] ---- [ui] ui/lint/lint-type-overflow2.rs stdout ----
[00:50:27] diff of stderr:
[00:50:27] 
[00:50:27] 34 LL |     let x =  1.7976931348623159e+308_f64; //~ warn: literal out of range for f64
[00:50:27] 36 
[00:50:27] - warning: attempt to negate with overflow
[00:50:27] + warning: this expression will panic at runtime
[00:50:27] 38   --> $DIR/lint-type-overflow2.rs:19:18
[00:50:27] 38   --> $DIR/lint-type-overflow2.rs:19:18
[00:50:27] 39    |
[00:50:27] 40 LL |     let x2: i8 = --128; //~ warn: literal out of range for i8
[00:50:27] -    |                  ^^^^^
[00:50:27] +    |                  ^^^^^ attempt to negate with overflow
[00:50:27] 42    |
[00:50:27] 43 note: lint level defined here
[00:50:27] 43 note: lint level defined here
[00:50:27] 44   --> $DIR/lint-type-overflow2.rs:13:9
[00:50:27] 
[00:50:27] 45    |
[00:50:27] 46 LL | #![warn(const_err)]
[00:50:27] - 
[00:50:27] - warning: this expression will panic at runtime
[00:50:27] -   --> $DIR/lint-type-overflow2.rs:19:18
[00:50:27] -    |
[00:50:27] -    |
[00:50:27] - LL |     let x2: i8 = --128; //~ warn: literal out of range for i8
[00:50:27] -    |                  ^^^^^ attempt to negate with overflow
[00:50:27] 55 
[00:50:27] 
[00:50:27] 
[00:50:27] The actual stderr differed from the expected stderr.
[00:50:27] The actual stderr differed from the expected stderr.
[00:50:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/lint-type-overflow2.stderr
[00:50:27] To update references, rerun the tests and pass the `--bless` flag
[00:50:27] To only update this specific test, also pass `--test-args lint/lint-type-overflow2.rs`
[00:50:27] error: 1 errors occurred comparing output.
[00:50:27] status: exit code: 0
[00:50:27] status: exit code: 0
[00:50:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-hiletest/src/runtest.rs:3284:9
[00:50:27] 
[00:50:27] 
[00:50:27] failures:
[00:50:27]     [ui] ui/lint/lint-type-overflow2.rs
[00:50:27]     [ui] ui/lint/lint-type-overflow2.rs
[00:50:27] 
[00:50:27] test result: FAILED. 4964 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[00:50:27] 
[00:50:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:50:27] 
[00:50:27] 
[00:50:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:27] 
[00:50:27] 
[00:50:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:27] Build completed unsuccessfully in 0:03:44
[00:50:27] Build completed unsuccessfully in 0:03:44
[00:50:27] Makefile:58: recipe for target 'check' failed
[00:50:27] make: *** [check] Error 1
33452 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
32644 ./src/llvm/test/tools
32464 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
32460 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
---
travis_time:end:038073ba:start=1541420480738318387,finish=1541420480744657403,duration=6339016
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1eec0dbe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cb01970
travis_time:start:0cb01970
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15532e39
$ dmesg | grep -i kill
