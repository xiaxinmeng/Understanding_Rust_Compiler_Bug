plain
travis_time:end:0020a8a1:start=1559365259953032784,finish=1559365348736358518,duration=88783325734
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:54:52] i................................................................................................... 1000/5609
[00:54:56] .................iiiii.............................................................................. 1100/5609
[00:54:59] .................................................................................................... 1200/5609
[00:55:01] .................................................................................................... 1300/5609
[00:55:04] .............................................................................................F...... 1400/5609
[00:55:10] .................................................................................................... 1600/5609
[00:55:13] .................................................i.................................................. 1700/5609
[00:55:16] .................................................................................................... 1800/5609
[00:55:20] .................................................................................................... 1900/5609
---
[00:57:46] 
[00:57:46] ---- [ui] ui/existential-type/issue-60371.rs stdout ----
[00:57:46] diff of stderr:
[00:57:46] 
[00:57:46] 17              <&() as Bug>
[00:57:46] 18    = note: the return type of a function must have a statically known size
[00:57:46] - error: could not find defining uses
[00:57:46] -   --> $DIR/issue-60371.rs:8:5
[00:57:46] -    |
[00:57:46] -    |
[00:57:46] - LL |     existential type Item: Bug;
[00:57:46] - 
[00:57:46] - error: aborting due to 3 previous errors
[00:57:46] + error: aborting due to 2 previous errors
[00:57:46] 27 
[00:57:46] 27 
[00:57:46] 28 Some errors have detailed explanations: E0277, E0658.
[00:57:46] 29 For more information about an error, try `rustc --explain E0277`.
[00:57:46] 
[00:57:46] 
[00:57:46] The actual stderr differed from the expected stderr.
[00:57:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/issue-60371.stderr
[00:57:46] To update references, rerun the tests and pass the `--bless` flag
[00:57:46] To only update this specific test, also pass `--test-args existential-type/issue-60371.rs`
[00:57:46] error: 1 errors occurred comparing output.
[00:57:46] status: exit code: 1
[00:57:46] status: exit code: 1
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential-type/issue-60371.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/auxiliary" "-A" "unused"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] error[E0658]: existential types are unstable
[00:57:46]   --> /checkout/src/test/ui/existential-type/issue-60371.rs:8:5
[00:57:46]    |
[00:57:46] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:57:46]    |
[00:57:46]    = note: for more information, see https://github.com/rust-lang/rust/issues/34511
[00:57:46]    = help: add #![feature(existential_type)] to the crate attributes to enable
[00:57:46] 
[00:57:46] 
[00:57:46] error[E0277]: the trait bound `(): Bug` is not satisfied
[00:57:46]    |
[00:57:46]    |
[00:57:46] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:57:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bug` is not implemented for `()`
[00:57:46]    = help: the following implementations were found:
[00:57:46]    = help: the following implementations were found:
[00:57:46]              <&() as Bug>
[00:57:46]    = note: the return type of a function must have a statically known size
[00:57:46] error: aborting due to 2 previous errors
[00:57:46] 
[00:57:46] Some errors have detailed explanations: E0277, E0658.
[00:57:46] For more information about an error, try `rustc --explain E0277`.
---
[00:57:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:57:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:57:46] 
[00:57:46] 
[00:57:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:46] 
[00:57:46] 
[00:57:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:46] Build completed unsuccessfully in 0:53:40
