plain
travis_time:end:31e52f32:start=1558345487257250998,finish=1558345572431079970,duration=85173828972
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:10:47] .................................................................................................... 4400/5533
[01:10:51] .................................................................................................... 4500/5533
[01:10:54] .................................................................................................... 4600/5533
[01:10:58] .................................................................................................... 4700/5533
[01:11:04] .......................................................................................F............ 4800/5533
[01:11:11] .................................................................................................... 5000/5533
[01:11:16] .................................................................................................... 5100/5533
[01:11:19] .................................................................................................... 5200/5533
[01:11:22] .................................................................................................... 5300/5533
---
[01:11:29] 1 error[E0106]: missing lifetime specifier
[01:11:29] -   --> $DIR/arbitrary_self_types_lifetime-2.rs:7:39
[01:11:29] +   --> $DIR/arbitrary_self_types_inexact_lifetime.rs:7:39
[01:11:29] 3    |
[01:11:29] - LL |     fn b(self: &Box<Foo>, f: &Foo) -> &Foo { f }
[01:11:29] + LL |     fn a(self: &Box<Foo>, f: &Foo) -> &Foo { f }
[01:11:29] 6    |
[01:11:29] 6    |
[01:11:29] 7    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
[01:11:29] 8 
[01:11:29] 9 error[E0106]: missing lifetime specifier
[01:11:29] -   --> $DIR/arbitrary_self_types_lifetime-2.rs:9:39
[01:11:29] +   --> $DIR/arbitrary_self_types_inexact_lifetime.rs:9:39
[01:11:29] +   --> $DIR/arbitrary_self_types_inexact_lifetime.rs:9:39
[01:11:29] 11    |
[01:11:29] - LL |     fn c(self: &Box<Foo>, f: &Foo) -> &Box<Foo> { self }
[01:11:29] + LL |     fn b(self: &Box<Foo>, f: &Foo) -> &Box<Foo> { self }
[01:11:29] 14    |
[01:11:29] 14    |
[01:11:29] 15    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
[01:11:29] 16 
[01:11:29] 17 error[E0106]: missing lifetime specifier
[01:11:29] -   --> $DIR/arbitrary_self_types_lifetime-2.rs:11:39
[01:11:29] +   --> $DIR/arbitrary_self_types_inexact_lifetime.rs:11:39
[01:11:29] +   --> $DIR/arbitrary_self_types_inexact_lifetime.rs:11:39
[01:11:29] 19    |
[01:11:29] - LL |     fn d(this: &Box<Foo>, f: &Foo) -> &Foo { f }
[01:11:29] + LL |     fn c(this: &Box<Foo>, f: &Foo) -> &Foo { f }
[01:11:29] 22    |
[01:11:29] 22    |
[01:11:29] 23    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `this` or `f`
[01:11:29] 
[01:11:29] The actual stderr differed from the expected stderr.
[01:11:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_inexact_lifetime/arbitrary_self_types_inexact_lifetime.stderr
[01:11:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_inexact_lifetime/arbitrary_self_types_inexact_lifetime.stderr
[01:11:29] To update references, rerun the tests and pass the `--bless` flag
[01:11:29] To only update this specific test, also pass `--test-args self/arbitrary_self_types_inexact_lifetime.rs`
[01:11:29] error: 1 errors occurred comparing output.
[01:11:29] status: exit code: 1
[01:11:29] status: exit code: 1
[01:11:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_inexact_lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_inexact_lifetime" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_inexact_lifetime/auxiliary" "-A" "unused"
[01:11:29] ------------------------------------------
[01:11:29] 
[01:11:29] ------------------------------------------
[01:11:29] stderr:
[01:11:29] stderr:
[01:11:29] ------------------------------------------
[01:11:29] error[E0106]: missing lifetime specifier
[01:11:29]   --> /checkout/src/test/ui/self/arbitrary_self_types_inexact_lifetime.rs:7:39
[01:11:29]    |
[01:11:29] LL |     fn a(self: &Box<Foo>, f: &Foo) -> &Foo { f } //~ ERROR E0106
[01:11:29]    |
[01:11:29]    |
[01:11:29]    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
[01:11:29] error[E0106]: missing lifetime specifier
[01:11:29]   --> /checkout/src/test/ui/self/arbitrary_self_types_inexact_lifetime.rs:9:39
[01:11:29]    |
[01:11:29]    |
[01:11:29] LL |     fn b(self: &Box<Foo>, f: &Foo) -> &Box<Foo> { self } //~ ERROR E0106
[01:11:29]    |
[01:11:29]    |
[01:11:29]    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
[01:11:29] error[E0106]: missing lifetime specifier
[01:11:29]   --> /checkout/src/test/ui/self/arbitrary_self_types_inexact_lifetime.rs:11:39
[01:11:29]    |
[01:11:29]    |
[01:11:29] LL |     fn c(this: &Box<Foo>, f: &Foo) -> &Foo { f } //~ ERROR E0106
[01:11:29]    |
[01:11:29]    |
[01:11:29]    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `this` or `f`
[01:11:29] error: aborting due to 3 previous errors
[01:11:29] 
[01:11:29] For more information about this error, try `rustc --explain E0106`.
[01:11:29] 
---
[01:11:29] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:11:29] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:29] 
[01:11:29] 
[01:11:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:29] 
[01:11:29] 
[01:11:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:29] Build completed unsuccessfully in 0:04:37
[01:11:29] Build completed unsuccessfully in 0:04:37
[01:11:29] make: *** [check] Error 1
[01:11:29] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05540370
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 20 10:57:50 UTC 2019
---
travis_time:end:04f74f1e:start=1558349872089340102,finish=1558349872094391169,duration=5051067
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06186118
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:246093c4
travis_time:start:246093c4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:35fb4a88
$ dmesg | grep -i kill
