plain
travis_time:end:13032e8f:start=1559116996725160297,finish=1559116997549758424,duration=824598127
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:08:43] ...................................................................................................i 500/5599
[01:08:47] .................................................................................................... 600/5599
[01:08:51] .................................................................................................... 700/5599
[01:08:56] .................................................................................................... 800/5599
[01:09:00] ..........F..........................................................................i...........i.. 900/5599
[01:09:08] ..............iiiii................................................................................. 1100/5599
[01:09:11] .................................................................................................... 1200/5599
[01:09:13] .................................................................................................... 1300/5599
[01:09:16] .................................................................................................... 1400/5599
---
[01:11:58] diff of stderr:
[01:11:58] 
[01:11:58] 5    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:11:58] 6    |
[01:11:58] 7 note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
[01:11:58] -   --> $SRC_DIR/libcore/mem.rs:LL:COL
[01:11:58] +   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
[01:11:58] 10 LL |     intrinsics::size_of::<T>()
[01:11:58] 11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:11:58] 
[01:11:58] 
[01:11:58] 
[01:11:58] The actual stderr differed from the expected stderr.
[01:11:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[01:11:58] To update references, rerun the tests and pass the `--bless` flag
[01:11:58] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[01:11:58] error: 1 errors occurred comparing output.
[01:11:58] status: exit code: 1
[01:11:58] status: exit code: 1
[01:11:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
[01:11:58] ------------------------------------------
[01:11:58] 
[01:11:58] ------------------------------------------
[01:11:58] stderr:
[01:11:58] stderr:
[01:11:58] ------------------------------------------
[01:11:58] error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
[01:11:58]    |
[01:11:58]    |
[01:11:58] LL |     bytes: [u8; std::mem::size_of::<Foo>()]
[01:11:58]    |
[01:11:58]    |
[01:11:58] note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
[01:11:58]   --> /checkout/src/libcore/mem/mod.rs:243:5
[01:11:58] LL |     intrinsics::size_of::<T>()
[01:11:58]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:11:58]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:11:58]    = note: ...which requires computing layout of `Foo`...
[01:11:58]    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
[01:11:58] note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
[01:11:58]    |
[01:11:58]    |
[01:11:58] LL |     bytes: [u8; std::mem::size_of::<Foo>()]
[01:11:58]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:11:58]    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
[01:11:58] note: cycle used when processing `Foo`
[01:11:58]    |
[01:11:58] LL | struct Foo {
[01:11:58]    | ^^^^^^^^^^
[01:11:58] 
---
[01:11:58] 
[01:11:58] ---- [ui] ui/type_length_limit.rs stdout ----
[01:11:58] diff of stderr:
[01:11:58] 
[01:11:58] 1 error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
[01:11:58] -   --> $SRC_DIR/libcore/mem.rs:LL:COL
[01:11:58] +   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
[01:11:58] 3    |
[01:11:58] 4 LL | pub fn drop<T>(_x: T) { }
[01:11:58] 
[01:11:58] 
[01:11:58] The actual stderr differed from the expected stderr.
[01:11:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
[01:11:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
[01:11:58] To update references, rerun the tests and pass the `--bless` flag
[01:11:58] To only update this specific test, also pass `--test-args type_length_limit.rs`
[01:11:58] error: 1 errors occurred comparing output.
[01:11:58] status: exit code: 1
[01:11:58] status: exit code: 1
[01:11:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary" "-A" "unused"
[01:11:58] ------------------------------------------
[01:11:58] 
[01:11:58] ------------------------------------------
[01:11:58] stderr:
[01:11:58] stderr:
[01:11:58] ------------------------------------------
[01:11:58] error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
[01:11:58]   --> /checkout/src/libcore/mem/mod.rs:628:1
[01:11:58]    |
[01:11:58] LL | pub fn drop<T>(_x: T) { }
[01:11:58]    |
[01:11:58]    |
[01:11:58]    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
[01:11:58] error: aborting due to previous error
[01:11:58] 
[01:11:58] 
[01:11:58] ------------------------------------------
---
[01:11:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:11:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:58] 
[01:11:58] 
[01:11:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:58] 
[01:11:58] 
[01:11:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:58] Build completed unsuccessfully in 0:04:47
[01:11:58] Build completed unsuccessfully in 0:04:47
[01:11:58] Makefile:48: recipe for target 'check' failed
[01:11:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08402e17
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 29 09:15:26 UTC 2019
---
travis_time:end:0308a102:start=1559121328360127205,finish=1559121328365972889,duration=5845684
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:054459b8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:147745c8
travis_time:start:147745c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d0abad0
$ dmesg | grep -i kill
