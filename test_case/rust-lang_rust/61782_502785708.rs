plain
travis_time:end:2bdbdaa6:start=1560790498964597622,finish=1560790499813618207,duration=849020585
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:04] .................................................................................................... 2500/5680
[00:55:08] .................................................................................................... 2600/5680
[00:55:12] .................................................................................................... 2700/5680
[00:55:16] .................................................................................................... 2800/5680
[00:55:20] ....................................................................................F............... 2900/5680
[00:55:28] .................................................................................................... 3100/5680
[00:55:30] .................................................................................................... 3200/5680
[00:55:34] .................................................................................................... 3300/5680
[00:55:38] .........................................................................................i.......... 3400/5680
[00:55:38] .........................................................................................i.......... 3400/5680
[00:55:41] .................................................................................................... 3500/5680
[00:55:44] ...............................................................ii...i..ii........................... 3600/5680
[00:55:48] .................................................................................................... 3700/5680
[00:55:52] .................................................................................................... 3800/5680
[00:55:55] .........................................................................ii...................F..... 3900/5680
[00:56:00] .................................................................................................... 4100/5680
[00:56:02] ..........................................................i......................................... 4200/5680
[00:56:04] .................................................................................................... 4300/5680
[00:56:13] .................................................................................................... 4400/5680
---
[00:57:05] diff of stderr:
[00:57:05] 
[00:57:05] 2   --> $DIR/issue-4736.rs:4:26
[00:57:05] 3    |
[00:57:05] 4 LL |     let z = NonCopyable{ p: () };
[00:57:05] -    |                          |
[00:57:05] -    |                          field does not exist
[00:57:05] -    |                          field does not exist
[00:57:05] -    |                          `NonCopyable` is a tuple struct, use the appropriate syntax: `NonCopyable(/* fields */)`
[00:57:05] +    |                          ^ help: a field with a similar name exists: `0`
[00:57:05] 10 error: aborting due to previous error
[00:57:05] 11 
[00:57:05] 
[00:57:05] 
[00:57:05] 
[00:57:05] The actual stderr differed from the expected stderr.
[00:57:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4736/issue-4736.stderr
[00:57:05] To update references, rerun the tests and pass the `--bless` flag
[00:57:05] To only update this specific test, also pass `--test-args issues/issue-4736.rs`
[00:57:05] error: 1 errors occurred comparing output.
[00:57:05] status: exit code: 1
[00:57:05] status: exit code: 1
[00:57:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-4736.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4736" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4736/auxiliary" "-A" "unused"
[00:57:05] ------------------------------------------
[00:57:05] 
[00:57:05] ------------------------------------------
[00:57:05] stderr:
[00:57:05] stderr:
[00:57:05] ------------------------------------------
[00:57:05] error[E0560]: struct `NonCopyable` has no field named `p`
[00:57:05]   --> /checkout/src/test/ui/issues/issue-4736.rs:4:26
[00:57:05]    |
[00:57:05] LL |     let z = NonCopyable{ p: () }; //~ ERROR struct `NonCopyable` has no field named `p`
[00:57:05]    |                          ^ help: a field with a similar name exists: `0`
[00:57:05] error: aborting due to previous error
[00:57:05] 
[00:57:05] For more information about this error, try `rustc --explain E0560`.
[00:57:05] 
---
[00:57:05] diff of stderr:
[00:57:05] 
[00:57:05] 2   --> $DIR/numeric-fields.rs:4:15
[00:57:05] 3    |
[00:57:05] 4 LL |     let s = S{0b1: 10, 0: 11};
[00:57:05] -    |               |
[00:57:05] -    |               field does not exist
[00:57:05] -    |               field does not exist
[00:57:05] -    |               `S` is a tuple struct, use the appropriate syntax: `S(/* fields */)`
[00:57:05] +    |               ^^^ `S` does not have this field
[00:57:05] +    = note: available fields are: `0`, `1`
[00:57:05] 9 
[00:57:05] 10 error[E0026]: struct `S` does not have a field named `0x1`
[00:57:05] 11   --> $DIR/numeric-fields.rs:7:17
[00:57:05] 11   --> $DIR/numeric-fields.rs:7:17
[00:57:05] 
[00:57:05] 
[00:57:05] The actual stderr differed from the expected stderr.
[00:57:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-fields/numeric-fields.stderr
[00:57:05] To update references, rerun the tests and pass the `--bless` flag
[00:57:05] To only update this specific test, also pass `--test-args numeric/numeric-fields.rs`
[00:57:05] error: 1 errors occurred comparing output.
[00:57:05] status: exit code: 1
[00:57:05] status: exit code: 1
[00:57:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numeric/numeric-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-fields" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-fields/auxiliary" "-A" "unused"
[00:57:05] ------------------------------------------
[00:57:05] 
[00:57:05] ------------------------------------------
[00:57:05] stderr:
[00:57:05] stderr:
[00:57:05] ------------------------------------------
[00:57:05] error[E0560]: struct `S` has no field named `0b1`
[00:57:05]   --> /checkout/src/test/ui/numeric/numeric-fields.rs:4:15
[00:57:05]    |
[00:57:05] LL |     let s = S{0b1: 10, 0: 11};
[00:57:05]    |               ^^^ `S` does not have this field
[00:57:05]    = note: available fields are: `0`, `1`
[00:57:05] 
[00:57:05] error[E0026]: struct `S` does not have a field named `0x1`
[00:57:05]   --> /checkout/src/test/ui/numeric/numeric-fields.rs:7:17
[00:57:05]   --> /checkout/src/test/ui/numeric/numeric-fields.rs:7:17
[00:57:05]    |
[00:57:05] LL |         S{0: a, 0x1: b, ..} => {}
[00:57:05]    |                 ^^^ struct `S` does not have this field
[00:57:05] error: aborting due to 2 previous errors
[00:57:05] 
[00:57:05] Some errors have detailed explanations: E0026, E0560.
[00:57:05] For more information about an error, try `rustc --explain E0026`.
---
[00:57:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:57:05] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:57:05] 
[00:57:05] 
[00:57:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:05] 
[00:57:05] 
[00:57:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:05] Build completed unsuccessfully in 0:51:45
---
travis_time:end:13edcdea:start=1560793938110075137,finish=1560793938114627696,duration=4552559
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0531e046
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3419c3b4
travis_time:start:3419c3b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:37bada43
$ dmesg | grep -i kill
