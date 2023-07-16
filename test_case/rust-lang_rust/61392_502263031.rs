plain
travis_time:end:384fdcdf:start=1560542942419741944,finish=1560542944616649614,duration=2196907670
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:53:24] ...................................................................iiiii............................ 1100/5679
[00:53:28] .................................................................................................... 1200/5679
[00:53:30] .................................................................................................... 1300/5679
[00:53:32] .................................................................................................... 1400/5679
[00:53:35] ............................................F....................................................... 1500/5679
[00:53:40] .................................................................................................... 1700/5679
[00:53:44] ..i................................................................................................. 1800/5679
[00:53:47] .................................................................................................... 1900/5679
[00:53:51] .................................................................................................... 2000/5679
---
[00:56:07] 
[00:56:07] ---- [ui] ui/existential-type/issue-60371.rs stdout ----
[00:56:07] diff of stderr:
[00:56:07] 
[00:56:07] 17              <&() as Bug>
[00:56:07] 18    = note: the return type of a function must have a statically known size
[00:56:07] - error: could not find defining uses
[00:56:07] -   --> $DIR/issue-60371.rs:8:5
[00:56:07] -    |
[00:56:07] -    |
[00:56:07] - LL |     existential type Item: Bug;
[00:56:07] - 
[00:56:07] - error: aborting due to 3 previous errors
[00:56:07] + error: aborting due to 2 previous errors
[00:56:07] 27 
[00:56:07] 27 
[00:56:07] 28 Some errors have detailed explanations: E0277, E0658.
[00:56:07] 29 For more information about an error, try `rustc --explain E0277`.
[00:56:07] 
[00:56:07] 
[00:56:07] The actual stderr differed from the expected stderr.
[00:56:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/issue-60371.stderr
[00:56:07] To update references, rerun the tests and pass the `--bless` flag
[00:56:07] To only update this specific test, also pass `--test-args existential-type/issue-60371.rs`
[00:56:07] error: 1 errors occurred comparing output.
[00:56:07] status: exit code: 1
[00:56:07] status: exit code: 1
[00:56:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential-type/issue-60371.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/auxiliary" "-A" "unused"
[00:56:07] ------------------------------------------
[00:56:07] 
[00:56:07] ------------------------------------------
[00:56:07] stderr:
[00:56:07] stderr:
[00:56:07] ------------------------------------------
[00:56:07] error[E0658]: existential types are unstable
[00:56:07]   --> /checkout/src/test/ui/existential-type/issue-60371.rs:8:5
[00:56:07]    |
[00:56:07] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:56:07]    |
[00:56:07]    = note: for more information, see https://github.com/rust-lang/rust/issues/34511
[00:56:07]    = help: add #![feature(existential_type)] to the crate attributes to enable
[00:56:07] 
[00:56:07] 
[00:56:07] error[E0277]: the trait bound `(): Bug` is not satisfied
[00:56:07]    |
[00:56:07]    |
[00:56:07] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:56:07]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bug` is not implemented for `()`
[00:56:07]    = help: the following implementations were found:
[00:56:07]    = help: the following implementations were found:
[00:56:07]              <&() as Bug>
[00:56:07]    = note: the return type of a function must have a statically known size
[00:56:07] error: aborting due to 2 previous errors
[00:56:07] 
[00:56:07] Some errors have detailed explanations: E0277, E0658.
[00:56:07] For more information about an error, try `rustc --explain E0277`.
---
[00:56:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:56:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:56:07] 
[00:56:07] 
[00:56:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:07] 
[00:56:07] 
[00:56:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:07] Build completed unsuccessfully in 0:51:35
---
travis_time:end:0091d1f6:start=1560546324514451452,finish=1560546324520986520,duration=6535068
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:079707d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f0e66e5
$ dmesg | grep -i kill
