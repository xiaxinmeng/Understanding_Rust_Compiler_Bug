plain
travis_time:end:172b6927:start=1558910389414951652,finish=1558910390215704661,duration=800753009
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:43] 
[01:11:43] running 5589 tests
[01:11:46] .........................................................................F..........F............... 100/5589
[01:11:56] .................................................................................................... 300/5589
[01:11:59] .................................................................................................... 400/5589
[01:12:03] ...................................................................................................i 500/5589
[01:12:07] .................................................................................................... 600/5589
---
[01:12:42] .................................................................................................... 1600/5589
[01:12:45] .............................................i...................................................... 1700/5589
[01:12:49] .................................................................................................... 1800/5589
[01:12:53] .................................................................................................... 1900/5589
[01:12:56] .............................................................................F...................... 2000/5589
[01:13:04] .................................................................................................... 2200/5589
[01:13:08] .................................................................................................... 2300/5589
[01:13:12] .................................................................................................... 2400/5589
[01:13:16] .................................................................................................... 2500/5589
---
[01:14:12] ....................i............................................................................... 4000/5589
[01:14:14] ....................................................................................i............... 4100/5589
[01:14:16] .................................................................................................... 4200/5589
[01:14:21] .................................................................................................... 4300/5589
[01:14:32] .............................................................................F...................... 4400/5589
[01:14:38] .................................................................................................... 4600/5589
[01:14:43] .................................................................................................... 4700/5589
[01:14:50] .................................................................................................... 4800/5589
[01:14:56] .................................................................................................... 4900/5589
---
[01:15:20] 
[01:15:20] 
[01:15:20] The actual stderr differed from the expected stderr.
[01:15:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding/associated-types-overridden-binding.stderr
[01:15:20] To update references, rerun the tests and pass the `--bless` flag
[01:15:20] To only update this specific test, also pass `--test-args associated-types/associated-types-overridden-binding.rs`
[01:15:20] error: 1 errors occurred comparing output.
[01:15:20] status: exit code: 1
[01:15:20] status: exit code: 1
[01:15:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding/auxiliary" "-A" "unused"
[01:15:20] ------------------------------------------
[01:15:20] 
[01:15:20] ------------------------------------------
[01:15:20] stderr:
[01:15:20] stderr:
[01:15:20] ------------------------------------------
[01:15:20] error[E0284]: type annotations required: cannot resolve `<Self as std::iter::Iterator>::Item == i32`
[01:15:20]    |
[01:15:20]    |
[01:15:20] LL | trait Bar: Foo<Item = u32> {} //~ ERROR type annotations required
[01:15:20]    |
[01:15:20] note: required by `Foo`
[01:15:20]   --> /checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs:3:1
[01:15:20]    |
[01:15:20]    |
[01:15:20] LL | trait Foo: Iterator<Item = i32> {}
[01:15:20] 
[01:15:20] error: aborting due to previous error
[01:15:20] 
[01:15:20] For more information about this error, try `rustc --explain E0284`.
---
[01:15:20] 9 
[01:15:20] 
[01:15:20] 
[01:15:20] The actual stderr differed from the expected stderr.
[01:15:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/associated-types-unconstrained.stderr
[01:15:20] To update references, rerun the tests and pass the `--bless` flag
[01:15:20] To only update this specific test, also pass `--test-args associated-types/associated-types-unconstrained.rs`
[01:15:20] error: 1 errors occurred comparing output.
[01:15:20] status: exit code: 1
[01:15:20] status: exit code: 1
[01:15:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-unconstrained.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/auxiliary" "-A" "unused"
[01:15:20] ------------------------------------------
[01:15:20] 
[01:15:20] ------------------------------------------
[01:15:20] stderr:
[01:15:20] stderr:
[01:15:20] ------------------------------------------
[01:15:20] error[E0284]: type annotations required: cannot resolve `<_ as Foo>::A == _`
[01:15:20]    |
[01:15:20]    |
[01:15:20] LL |     let x: isize = Foo::bar();
[01:15:20] 
[01:15:20] error: aborting due to previous error
[01:15:20] 
[01:15:20] For more information about this error, try `rustc --explain E0284`.
---
[01:15:20] 
[01:15:20] 
[01:15:20] The actual stderr differed from the expected stderr.
[01:15:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
[01:15:20] To update references, rerun the tests and pass the `--bless` flag
[01:15:20] To only update this specific test, also pass `--test-args issues/issue-12028.rs`
[01:15:20] error: 1 errors occurred comparing output.
[01:15:20] status: exit code: 1
[01:15:20] status: exit code: 1
[01:15:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12028.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/auxiliary" "-A" "unused"
[01:15:20] ------------------------------------------
[01:15:20] 
[01:15:20] ------------------------------------------
[01:15:20] stderr:
[01:15:20] stderr:
[01:15:20] ------------------------------------------
[01:15:20] error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
[01:15:20]    |
[01:15:20]    |
[01:15:20] LL |         self.input_stream(&mut stream); //~ ERROR type annotations required
[01:15:20] 
[01:15:20] error: aborting due to previous error
[01:15:20] 
[01:15:20] For more information about this error, try `rustc --explain E0284`.
---
[01:15:20] 9 
[01:15:20] 
[01:15:20] 
[01:15:20] The actual stderr differed from the expected stderr.
[01:15:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer/question-mark-type-infer.stderr
[01:15:20] To update references, rerun the tests and pass the `--bless` flag
[01:15:20] To only update this specific test, also pass `--test-args question-mark-type-infer.rs`
[01:15:20] error: 1 errors occurred comparing output.
[01:15:20] status: exit code: 1
[01:15:20] status: exit code: 1
[01:15:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/question-mark-type-infer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer/auxiliary" "-A" "unused"
[01:15:20] ------------------------------------------
[01:15:20] 
[01:15:20] ------------------------------------------
[01:15:20] stderr:
[01:15:20] stderr:
[01:15:20] ------------------------------------------
[01:15:20] error[E0284]: type annotations required: cannot resolve `<_ as std::ops::Try>::Ok == _`
[01:15:20]   --> /checkout/src/test/ui/question-mark-type-infer.rs:12:5
[01:15:20]    |
[01:15:20] LL |     l.iter().map(f).collect()? //~ ERROR type annotations required: cannot resolve
[01:15:20] 
[01:15:20] error: aborting due to previous error
[01:15:20] 
[01:15:20] For more information about this error, try `rustc --explain E0284`.
---
[01:15:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:15:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:20] 
[01:15:20] 
[01:15:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:20] 
[01:15:20] 
[01:15:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:20] Build completed unsuccessfully in 0:04:56
[01:15:20] Build completed unsuccessfully in 0:04:56
[01:15:20] Makefile:48: recipe for target 'check' failed
[01:15:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cc3c487
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 23:55:22 UTC 2019
