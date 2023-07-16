plain
travis_time:end:03904493:start=1547273700804221930,finish=1547273702362536287,duration=1558314357
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:10:20] .................................................................................................... 1300/2951
[01:10:33] .................................................................................................... 1400/2951
[01:10:46] .................................................................................................... 1500/2951
[01:10:56] ......................................................................i............................. 1600/2951
[01:11:12] ........................F........................................................................... 1700/2951
[01:11:37] .................................................................................................... 1900/2951
[01:11:54] .i......................................................................i........................... 2000/2951
[01:12:22] .................................................................................................... 2100/2951
[01:12:39] ...............................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
---
[01:14:49] normalized stderr:
[01:14:49] warning: unreachable pattern
[01:14:49]   --> $DIR/issue-57472.rs:9:9
[01:14:49]    |
[01:14:49] LL |         Punned { bar: [_], .. } => println!("bar"),
[01:14:49]    |
[01:14:49]    = note: #[warn(unreachable_patterns)] on by default
[01:14:49] 
[01:14:49] 
[01:14:49] 
[01:14:49] 
[01:14:49] 
[01:14:49] The actual stderr differed from the expected stderr.
[01:14:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-57472/issue-57472.stderr
[01:14:49] To update references, rerun the tests and pass the `--bless` flag
[01:14:49] To only update this specific test, also pass `--test-args issues/issue-57472.rs`
[01:14:49] error: 1 errors occurred comparing output.
[01:14:49] status: exit code: 0
[01:14:49] status: exit code: 0
[01:14:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-57472.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-57472/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-57472/auxiliary"
[01:14:49] ------------------------------------------
[01:14:49] 
[01:14:49] ------------------------------------------
[01:14:49] stderr:
[01:14:49] stderr:
[01:14:49] ------------------------------------------
[01:14:49] {"message":"unreachable pattern","code":{"code":"unreachable_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-57472.rs","byte_start":168,"byte_end":191,"line_start":9,"line_end":9,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        Punned { bar: [_], .. } => println!(\"bar\"),","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_patterns)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unreachable pattern\n  --> /checkout/src/test/run-pass/issues/issue-57472.rs:9:9\n   |\nLL |         Punned { bar: [_], .. } => println!(\"bar\"),\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unreachable_patterns)] on by default\n\n"}
[01:14:49] ------------------------------------------
[01:14:49] 
[01:14:49] thread '[run-pass] run-pass/issues/issue-57472.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:14:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:14:49] 
[01:14:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:14:49] 
[01:14:49] 
[01:14:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:49] 
[01:14:49] 
[01:14:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:49] Build completed unsuccessfully in 0:11:41
[01:14:49] Build completed unsuccessfully in 0:11:41
[01:14:49] make: *** [check] Error 1
[01:14:49] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b3b6890
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 12 07:30:03 UTC 2019
---
travis_time:end:07309e53:start=1547278204673188618,finish=1547278204723789343,duration=50600725
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:098332e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05525fc0
$ dmesg | grep -i kill
