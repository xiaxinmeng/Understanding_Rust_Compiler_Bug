plain
travis_time:end:1450986c:start=1560539481888136712,finish=1560539482720525909,duration=832389197
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:58:47] ...................................................................iiiii............................ 1100/5679
[00:58:51] .................................................................................................... 1200/5679
[00:58:54] .................................................................................................... 1300/5679
[00:58:56] .................................................................................................... 1400/5679
[00:59:00] ............................................F....................................................... 1500/5679
[00:59:05] .................................................................................................... 1700/5679
[00:59:09] ..i................................................................................................. 1800/5679
[00:59:13] .................................................................................................... 1900/5679
[00:59:16] .................................................................................................... 2000/5679
---
[01:01:50] 
[01:01:50] ---- [ui] ui/existential-type/issue-60371.rs stdout ----
[01:01:50] diff of stderr:
[01:01:50] 
[01:01:50] 17              <&() as Bug>
[01:01:50] 18    = note: the return type of a function must have a statically known size
[01:01:50] - error: could not find defining uses
[01:01:50] -   --> $DIR/issue-60371.rs:8:5
[01:01:50] -    |
[01:01:50] -    |
[01:01:50] - LL |     existential type Item: Bug;
[01:01:50] - 
[01:01:50] - error: aborting due to 3 previous errors
[01:01:50] + error: aborting due to 2 previous errors
[01:01:50] 27 
[01:01:50] 27 
[01:01:50] 28 Some errors have detailed explanations: E0277, E0658.
[01:01:50] 29 For more information about an error, try `rustc --explain E0277`.
[01:01:50] 
[01:01:50] 
[01:01:50] The actual stderr differed from the expected stderr.
[01:01:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/issue-60371.stderr
[01:01:50] To update references, rerun the tests and pass the `--bless` flag
[01:01:50] To only update this specific test, also pass `--test-args existential-type/issue-60371.rs`
[01:01:50] error: 1 errors occurred comparing output.
[01:01:50] status: exit code: 1
[01:01:50] status: exit code: 1
[01:01:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential-type/issue-60371.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/auxiliary" "-A" "unused"
[01:01:50] ------------------------------------------
[01:01:50] 
[01:01:50] ------------------------------------------
[01:01:50] stderr:
[01:01:50] stderr:
[01:01:50] ------------------------------------------
[01:01:50] error[E0658]: existential types are unstable
[01:01:50]   --> /checkout/src/test/ui/existential-type/issue-60371.rs:8:5
[01:01:50]    |
[01:01:50] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[01:01:50]    |
[01:01:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/34511
[01:01:50]    = help: add #![feature(existential_type)] to the crate attributes to enable
[01:01:50] 
[01:01:50] 
[01:01:50] error[E0277]: the trait bound `(): Bug` is not satisfied
[01:01:50]    |
[01:01:50]    |
[01:01:50] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[01:01:50]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bug` is not implemented for `()`
[01:01:50]    = help: the following implementations were found:
[01:01:50]    = help: the following implementations were found:
[01:01:50]              <&() as Bug>
[01:01:50]    = note: the return type of a function must have a statically known size
[01:01:50] error: aborting due to 2 previous errors
[01:01:50] 
[01:01:50] Some errors have detailed explanations: E0277, E0658.
[01:01:50] For more information about an error, try `rustc --explain E0277`.
---
[01:01:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:01:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:01:50] 
[01:01:50] 
[01:01:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:50] 
[01:01:50] 
[01:01:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:50] Build completed unsuccessfully in 0:56:48
---
travis_time:end:022e8940:start=1560543206248598338,finish=1560543206253651431,duration=5053093
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06232d87
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08cf36ca
travis_time:start:08cf36ca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18f0ae78
$ dmesg | grep -i kill
