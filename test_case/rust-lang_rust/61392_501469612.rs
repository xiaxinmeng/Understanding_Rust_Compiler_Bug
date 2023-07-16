plain
travis_time:end:25cb411e:start=1560373042291018802,finish=1560373043106327394,duration=815308592
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:22] ..................................................................iiiii............................. 1100/5676
[00:55:25] .................................................................................................... 1200/5676
[00:55:28] .................................................................................................... 1300/5676
[00:55:30] .................................................................................................... 1400/5676
[00:55:33] ...........................................F........................................................ 1500/5676
[00:55:38] .................................................................................................... 1700/5676
[00:55:42] .i.................................................................................................. 1800/5676
[00:55:45] .................................................................................................... 1900/5676
[00:55:49] .................................................................................................... 2000/5676
---
[00:58:10] 
[00:58:10] ---- [ui] ui/existential-type/issue-60371.rs stdout ----
[00:58:10] diff of stderr:
[00:58:10] 
[00:58:10] 17              <&() as Bug>
[00:58:10] 18    = note: the return type of a function must have a statically known size
[00:58:10] - error: could not find defining uses
[00:58:10] -   --> $DIR/issue-60371.rs:8:5
[00:58:10] -    |
[00:58:10] -    |
[00:58:10] - LL |     existential type Item: Bug;
[00:58:10] - 
[00:58:10] - error: aborting due to 3 previous errors
[00:58:10] + error: aborting due to 2 previous errors
[00:58:10] 27 
[00:58:10] 27 
[00:58:10] 28 Some errors have detailed explanations: E0277, E0658.
[00:58:10] 29 For more information about an error, try `rustc --explain E0277`.
[00:58:10] 
[00:58:10] 
[00:58:10] The actual stderr differed from the expected stderr.
[00:58:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/issue-60371.stderr
[00:58:10] To update references, rerun the tests and pass the `--bless` flag
[00:58:10] To only update this specific test, also pass `--test-args existential-type/issue-60371.rs`
[00:58:10] error: 1 errors occurred comparing output.
[00:58:10] status: exit code: 1
[00:58:10] status: exit code: 1
[00:58:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential-type/issue-60371.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/auxiliary" "-A" "unused"
[00:58:10] ------------------------------------------
[00:58:10] 
[00:58:10] ------------------------------------------
[00:58:10] stderr:
[00:58:10] stderr:
[00:58:10] ------------------------------------------
[00:58:10] error[E0658]: existential types are unstable
[00:58:10]   --> /checkout/src/test/ui/existential-type/issue-60371.rs:8:5
[00:58:10]    |
[00:58:10] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:58:10]    |
[00:58:10]    = note: for more information, see https://github.com/rust-lang/rust/issues/34511
[00:58:10]    = help: add #![feature(existential_type)] to the crate attributes to enable
[00:58:10] 
[00:58:10] 
[00:58:10] error[E0277]: the trait bound `(): Bug` is not satisfied
[00:58:10]    |
[00:58:10]    |
[00:58:10] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:58:10]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bug` is not implemented for `()`
[00:58:10]    = help: the following implementations were found:
[00:58:10]    = help: the following implementations were found:
[00:58:10]              <&() as Bug>
[00:58:10]    = note: the return type of a function must have a statically known size
[00:58:10] error: aborting due to 2 previous errors
[00:58:10] 
[00:58:10] Some errors have detailed explanations: E0277, E0658.
[00:58:10] For more information about an error, try `rustc --explain E0277`.
---
[00:58:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:58:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:10] 
[00:58:10] 
[00:58:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:10] 
[00:58:10] 
[00:58:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:10] Build completed unsuccessfully in 0:53:20
