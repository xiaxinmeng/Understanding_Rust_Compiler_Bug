plain
travis_time:end:053bcad0:start=1557182845083268694,finish=1557182966259610676,duration=121176341982
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:33] .................................................................................................... 1300/5496
[01:09:36] .................................................................................................... 1400/5496
[01:09:39] .................................................................................................... 1500/5496
[01:09:41] .................................................................................................... 1600/5496
[01:09:44] ..................i...............F................................................................. 1700/5496
[01:09:52] .................................................................................................... 1900/5496
[01:09:55] .................................................................................................... 2000/5496
[01:09:59] ............................................................i....................................... 2100/5496
[01:10:02] .................................................................................................... 2200/5496
---
[01:12:07] 
[01:12:07] ---- [ui] ui/generator-yielding-or-returning-itself.rs stdout ----
[01:12:07] diff of stderr:
[01:12:07] 
[01:12:07] - error[E0271]: type mismatch resolving `<[generator@$DIR/generator-yielding-or-returning-itself.rs:15:34: 19:6 _] as std::ops::Generator>::Return == [generator@$DIR/generator-yielding-or-returning-itself.rs:15:34: 19:6 _]`
[01:12:07] + error[E0644]: closure/generator type that references itself
[01:12:07] +   --> $DIR/generator-yielding-or-returning-itself.rs:15:34
[01:12:07] 3    |
[01:12:07] 3    |
[01:12:07] - LL |     want_cyclic_generator_return(|| {
[01:12:07] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cyclic type of infinite size
[01:12:07] + LL |       want_cyclic_generator_return(|| {
[01:12:07] + LL | |
[01:12:07] + LL | |
[01:12:07] + LL | |         if false { yield None.unwrap(); }
[01:12:07] + LL | |         None.unwrap()
[01:12:07] + LL | |     })
[01:12:07] +    | |_____^ cyclic type of infinite size
[01:12:07] 7    = note: closures cannot capture themselves or take themselves as argument;
[01:12:07] 8            this error may be the result of a recent compiler bug-fix,
[01:12:07] 
[01:12:07] 9            see https://github.com/rust-lang/rust/issues/46062 for more details
[01:12:07] 9            see https://github.com/rust-lang/rust/issues/46062 for more details
[01:12:07] - note: required by `want_cyclic_generator_return`
[01:12:07] -    |
[01:12:07] -    |
[01:12:07] - LL | / pub fn want_cyclic_generator_return<T>(_: T)
[01:12:07] - LL | |     where T: Generator<Yield = (), Return = T>
[01:12:07] - LL | | {
[01:12:07] - LL | | }
[01:12:07] 18 
[01:12:07] 18 
[01:12:07] 19 error[E0271]: type mismatch resolving `<[generator@$DIR/generator-yielding-or-returning-itself.rs:28:33: 32:6 _] as std::ops::Generator>::Yield == [generator@$DIR/generator-yielding-or-returning-itself.rs:28:33: 32:6 _]`
[01:12:07] 
[01:12:07] 36 
[01:12:07] 37 error: aborting due to 2 previous errors
[01:12:07] 38 
[01:12:07] 38 
[01:12:07] - For more information about this error, try `rustc --explain E0271`.
[01:12:07] + Some errors have detailed explanations: E0271, E0644.
[01:12:07] + For more information about an error, try `rustc --explain E0271`.
[01:12:07] 40 
[01:12:07] 
[01:12:07] 
[01:12:07] The actual stderr differed from the expected stderr.
[01:12:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator-yielding-or-returning-itself/generator-yielding-or-returning-itself.stderr
[01:12:07] To update references, rerun the tests and pass the `--bless` flag
[01:12:07] To only update this specific test, also pass `--test-args generator-yielding-or-returning-itself.rs`
[01:12:07] error: 1 errors occurred comparing output.
[01:12:07] status: exit code: 1
[01:12:07] status: exit code: 1
[01:12:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator-yielding-or-returning-itself.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator-yielding-or-returning-itself/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator-yielding-or-returning-itself/auxiliary" "-A" "unused"
[01:12:07] ------------------------------------------
[01:12:07] 
[01:12:07] ------------------------------------------
[01:12:07] stderr:
[01:12:07] stderr:
[01:12:07] ------------------------------------------
[01:12:07] error[E0644]: closure/generator type that references itself
[01:12:07]   --> /checkout/src/test/ui/generator-yielding-or-returning-itself.rs:15:34
[01:12:07]    |
[01:12:07] LL |       want_cyclic_generator_return(|| {
[01:12:07] LL | |         //~^ ERROR type mismatch
[01:12:07] LL | |         //~^ ERROR type mismatch
[01:12:07] LL | |         if false { yield None.unwrap(); }
[01:12:07] LL | |         None.unwrap()
[01:12:07] LL | |     })
[01:12:07]    | |_____^ cyclic type of infinite size
[01:12:07]    = note: closures cannot capture themselves or take themselves as argument;
[01:12:07]            this error may be the result of a recent compiler bug-fix,
[01:12:07]            see https://github.com/rust-lang/rust/issues/46062 for more details
[01:12:07] 
[01:12:07] 
[01:12:07] error[E0271]: type mismatch resolving `<[generator@/checkout/src/test/ui/generator-yielding-or-returning-itself.rs:28:33: 32:6 _] as std::ops::Generator>::Yield == [generator@/checkout/src/test/ui/generator-yielding-or-returning-itself.rs:28:33: 32:6 _]`
[01:12:07]   --> /checkout/src/test/ui/generator-yielding-or-returning-itself.rs:28:5
[01:12:07]    |
[01:12:07] LL |     want_cyclic_generator_yield(|| {
[01:12:07]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cyclic type of infinite size
[01:12:07]    = note: closures cannot capture themselves or take themselves as argument;
[01:12:07]            this error may be the result of a recent compiler bug-fix,
[01:12:07]            see https://github.com/rust-lang/rust/issues/46062 for more details
[01:12:07]            see https://github.com/rust-lang/rust/issues/46062 for more details
[01:12:07] note: required by `want_cyclic_generator_yield`
[01:12:07]   --> /checkout/src/test/ui/generator-yielding-or-returning-itself.rs:22:1
[01:12:07]    |
[01:12:07] LL | / pub fn want_cyclic_generator_yield<T>(_: T)
[01:12:07] LL | |     where T: Generator<Yield = T, Return = ()>
[01:12:07] LL | | {
[01:12:07]    | |_^
[01:12:07] 
[01:12:07] error: aborting due to 2 previous errors
[01:12:07] 
---
[01:12:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:12:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:07] 
[01:12:07] 
[01:12:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:07] 
[01:12:07] 
[01:12:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:07] Build completed unsuccessfully in 0:04:24
[01:12:07] Build completed unsuccessfully in 0:04:24
[01:12:07] make: *** [check] Error 1
[01:12:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15a4686f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May  7 00:01:43 UTC 2019
---
travis_time:end:0f300312:start=1557187304438129444,finish=1557187304442791970,duration=4662526
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a016349
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ea2207a
travis_time:start:0ea2207a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a263902
$ dmesg | grep -i kill
