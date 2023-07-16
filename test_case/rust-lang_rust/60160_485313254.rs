plain
travis_time:end:227a7cca:start=1555899487790897001,finish=1555899574766109539,duration=86975212538
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:01:44] .................................................................................................... 1600/5549
[01:01:47] .............................................................i...................................... 1700/5549
[01:01:50] .................................................................................................... 1800/5549
[01:01:54] .................................................................................................... 1900/5549
[01:01:57] ........................................................F........................................... 2000/5549
[01:02:04] i................................................................................................... 2200/5549
[01:02:08] .................................................................................................... 2300/5549
[01:02:12] .................................................................................................... 2400/5549
[01:02:16] .................................................................................................... 2500/5549
---
[01:04:06] 12 LL |         let _ = if true {
[01:04:06] 13 LL |         });
[01:04:06] -    |           ^
[01:04:06] -    |           |
[01:04:06] -    |           help: `}` may belong here
[01:04:06] +    |           ^ help: `}` may belong here
[01:04:06] 18 error: expected identifier, found `;`
[01:04:06] 19   --> $DIR/issue-60075.rs:6:11
[01:04:06] 
[01:04:06] 
[01:04:06] 
[01:04:06] The actual stderr differed from the expected stderr.
[01:04:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-60075/issue-60075.stderr
[01:04:06] To update references, rerun the tests and pass the `--bless` flag
[01:04:06] To only update this specific test, also pass `--test-args issue-60075.rs`
[01:04:06] error: 1 errors occurred comparing output.
[01:04:06] status: exit code: 1
[01:04:06] status: exit code: 1
[01:04:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-60075.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-60075/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-60075/auxiliary" "-A" "unused"
[01:04:06] ------------------------------------------
[01:04:06] 
[01:04:06] ------------------------------------------
[01:04:06] stderr:
[01:04:06] stderr:
[01:04:06] ------------------------------------------
[01:04:06] error: expected one of `.`, `;`, `?`, `else`, or an operator, found `}`
[01:04:06]   --> /checkout/src/test/ui/issue-60075.rs:6:10
[01:04:06]    |
[01:04:06] LL |         });
[01:04:06]    |          ^ expected one of `.`, `;`, `?`, `else`, or an operator here
[01:04:06] 
[01:04:06] error: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`
[01:04:06]    |
[01:04:06]    |
[01:04:06] LL |     fn qux() -> Option<usize> {
[01:04:06] LL |         let _ = if true {
[01:04:06] LL |         });
[01:04:06] LL |         });
[01:04:06]    |           ^ help: `}` may belong here
[01:04:06] error: expected identifier, found `;`
[01:04:06]   --> /checkout/src/test/ui/issue-60075.rs:6:11
[01:04:06]    |
[01:04:06] LL |         });
[01:04:06] LL |         });
[01:04:06]    |           ^ expected identifier
[01:04:06] 
[01:04:06] error: missing `fn`, `type`, or `const` for trait-item declaration
[01:04:06]    |
[01:04:06] LL |           });
[01:04:06]    |  ____________^
[01:04:06]    |  ____________^
[01:04:06] LL | | //~^ ERROR expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`
[01:04:06] LL | | //~^^ ERROR expected one of `.`, `;`, `?`, `else`, or an operator, found `}`
[01:04:06] LL | | //~^^^ ERROR 6:11: 6:12: expected identifier, found `;`
[01:04:06] LL | | //~^^^^ ERROR missing `fn`, `type`, or `const` for trait-item declaration
[01:04:06] LL | |         Some(4)
[01:04:06]    | |________^ missing `fn`, `type`, or `const`
[01:04:06] error: aborting due to 4 previous errors
[01:04:06] 
[01:04:06] 
[01:04:06] ------------------------------------------
---
[01:04:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:04:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:06] 
[01:04:06] 
[01:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:06] 
[01:04:06] 
[01:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:06] Build completed unsuccessfully in 0:04:12
[01:04:06] Build completed unsuccessfully in 0:04:12
[01:04:06] make: *** [check] Error 1
[01:04:06] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:000820d7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 22 03:23:51 UTC 2019
---
travis_time:end:0466e6d8:start=1555903432284424674,finish=1555903432291287735,duration=6863061
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1699aeb0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:26fd3bd9
$ dmesg | grep -i kill
