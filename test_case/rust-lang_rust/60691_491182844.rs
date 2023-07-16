plain
travis_time:end:1db21a04:start=1557467457328255554,finish=1557467458137028168,duration=808772614
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:14:57] ........iiiii....................................................................................... 1100/5508
[01:15:00] .................................................................................................... 1200/5508
[01:15:02] .................................................................................................... 1300/5508
[01:15:05] .................................................................................................... 1400/5508
[01:15:08] ......................................................F............................................. 1500/5508
[01:15:15] .........................i.......................................................................... 1700/5508
[01:15:19] .................................................................................................... 1800/5508
[01:15:23] .................................................................................................... 1900/5508
[01:15:27] .................................................................................................... 2000/5508
---
[01:17:50] diff of stderr:
[01:17:50] 
[01:17:50] 2   --> $DIR/await-macro.rs:9:5
[01:17:50] 3    |
[01:17:50] 4 LL |     await!(bar());
[01:17:50] +    |     ^^^^^^^^^^^^^
[01:17:50] 6    |
[01:17:50] 7    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:17:50] 7    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:17:50] 8    = help: add #![feature(await_macro)] to the crate attributes to enable
[01:17:50] 
[01:17:50] The actual stderr differed from the expected stderr.
[01:17:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/await-macro/await-macro.stderr
[01:17:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/await-macro/await-macro.stderr
[01:17:50] To update references, rerun the tests and pass the `--bless` flag
[01:17:50] To only update this specific test, also pass `--test-args feature-gate/await-macro.rs`
[01:17:50] error: 1 errors occurred comparing output.
[01:17:50] status: exit code: 1
[01:17:50] status: exit code: 1
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/await-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/await-macro" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/await-macro/auxiliary" "-A" "unused"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] error[E0658]: `await!(<expr>)` macro syntax is unstable, and will soon be removed in favor of `<expr>.await` syntax.
[01:17:50]    |
[01:17:50]    |
[01:17:50] LL |     await!(bar()); //~ ERROR `await!(<expr>)` macro syntax is unstable, and will soon be removed
[01:17:50]    |
[01:17:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:17:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:17:50]    = help: add #![feature(await_macro)] to the crate attributes to enable
[01:17:50] error: aborting due to previous error
[01:17:50] 
[01:17:50] For more information about this error, try `rustc --explain E0658`.
[01:17:50] 
---
[01:17:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:17:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:50] 
[01:17:50] 
[01:17:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:50] 
[01:17:50] 
[01:17:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:50] Build completed unsuccessfully in 0:04:57
[01:17:50] Build completed unsuccessfully in 0:04:57
[01:17:50] make: *** [check] Error 1
[01:17:50] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08494d0d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 10 07:08:59 UTC 2019
