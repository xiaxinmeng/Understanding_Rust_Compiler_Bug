plain
travis_time:end:10f13040:start=1555633682557474072,finish=1555633793286792145,duration=110729318073
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:56] .................................................................................................... 1200/5546
[01:09:59] .................................................................................................... 1300/5546
[01:10:01] .................................................................................................... 1400/5546
[01:10:05] .................................................................................................... 1500/5546
[01:10:08] ...........................F........................................................................ 1600/5546
[01:10:15] .................................................................................................... 1800/5546
[01:10:19] .................................................................................................... 1900/5546
[01:10:23] .................................................................................................... 2000/5546
[01:10:27] ...................................................................................................i 2100/5546
---
[01:10:52] .................................................................................................... 2700/5546
[01:10:57] .................................................................................................... 2800/5546
[01:11:01] .................................................................................................... 2900/5546
[01:11:05] .................................................................................................... 3000/5546
[01:11:09] .....................................F.............................................................. 3100/5546
[01:11:17] .................................................................................................... 3300/5546
[01:11:21] ........................................i........................................................... 3400/5546
[01:11:25] .................................................................................................... 3500/5546
[01:11:29] ..............ii...i..ii............................................................................ 3600/5546
---
[01:12:15] .................................................................................................... 4700/5546
[01:12:22] .................................................................................................... 4800/5546
[01:12:26] .................................................................................................... 4900/5546
[01:12:29] .................................................................................................... 5000/5546
[01:12:34] .......................................................F............................................ 5100/5546
[01:12:42] .................................................................................................... 5300/5546
[01:12:45] .................................................................................................... 5400/5546
[01:12:48] ....................................................................................i............... 5500/5546
[01:12:50] ..............................................
---
[01:12:50] 56 
[01:12:50] 
[01:12:50] 
[01:12:50] The actual stderr differed from the expected stderr.
[01:12:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/feature-gate-async-await.stderr
[01:12:50] To update references, rerun the tests and pass the `--bless` flag
[01:12:50] To only update this specific test, also pass `--test-args feature-gates/feature-gate-async-await.rs`
[01:12:50] error: 1 errors occurred comparing output.
[01:12:50] status: exit code: 1
[01:12:50] status: exit code: 1
[01:12:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/auxiliary" "-A" "unused"
[01:12:50] ------------------------------------------
[01:12:50] 
[01:12:50] ------------------------------------------
[01:12:50] stderr:
[01:12:50] stderr:
[01:12:50] ------------------------------------------
[01:12:50] error[E0706]: trait fns cannot be declared `async`
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |     async fn foo(); //~ ERROR trait fns cannot be declared `async`
[01:12:50] 
[01:12:50] 
[01:12:50] error[E0658]: async fn is unstable
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |     async fn foo() {} //~ ERROR async fn is unstable
[01:12:50]    |
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = help: add #![feature(async_await)] to the crate attributes to enable
[01:12:50] 
[01:12:50] error[E0658]: async fn is unstable
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |     async fn foo(); //~ ERROR trait fns cannot be declared `async`
[01:12:50]    |
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = help: add #![feature(async_await)] to the crate attributes to enable
[01:12:50] 
[01:12:50] error[E0658]: async fn is unstable
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL | async fn foo() {} //~ ERROR async fn is unstable
[01:12:50]    |
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = help: add #![feature(async_await)] to the crate attributes to enable
[01:12:50] 
[01:12:50] error[E0658]: async blocks are unstable
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |     let _ = async {}; //~ ERROR async blocks are unstable
[01:12:50]    |
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = help: add #![feature(async_await)] to the crate attributes to enable
[01:12:50] 
[01:12:50] error[E0658]: async closures are unstable
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |     let _ = async || {}; //~ ERROR async closures are unstable
[01:12:50]    |
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:12:50]    = help: add #![feature(async_await)] to the crate attributes to enable
[01:12:50] error: aborting due to 6 previous errors
[01:12:50] 
[01:12:50] For more information about this error, try `rustc --explain E0658`.
[01:12:50] 
---
[01:12:50] 
[01:12:50] 16 
[01:12:50] 17 error: aborting due to 3 previous errors
[01:12:50] 18 
[01:12:50] - Some errors occurred: E0425, E0601.
[01:12:50] + Some errors have detailed explanations: E0425, E0601.
[01:12:50] 21 
[01:12:50] 
[01:12:50] 
[01:12:50] The actual stderr differed from the expected stderr.
[01:12:50] The actual stderr differed from the expected stderr.
[01:12:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/issue-60057.stderr
[01:12:50] To update references, rerun the tests and pass the `--bless` flag
[01:12:50] To only update this specific test, also pass `--test-args issues/issue-60057.rs`
[01:12:50] error: 1 errors occurred comparing output.
[01:12:50] status: exit code: 1
[01:12:50] status: exit code: 1
[01:12:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60057.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/auxiliary" "-A" "unused"
[01:12:50] ------------------------------------------
[01:12:50] 
[01:12:50] ------------------------------------------
[01:12:50] stderr:
[01:12:50] stderr:
[01:12:50] ------------------------------------------
[01:12:50] error[E0425]: cannot find value `banana` in this scope
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
[01:12:50]    |                     ^^^^^^ a field by this name exists in `Self`
[01:12:50] 
[01:12:50] error[E0425]: cannot find value `banana` in this scope
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
[01:12:50]    |                     ^^^^^^ help: you might have meant to use the available field: `self.banana`
[01:12:50] 
[01:12:50] error[E0601]: `main` function not found in crate `issue_60057`
[01:12:50]    |
[01:12:50]    = note: consider adding a `main` function to `/checkout/src/test/ui/issues/issue-60057.rs`
[01:12:50] error: aborting due to 3 previous errors
[01:12:50] 
[01:12:50] Some errors have detailed explanations: E0425, E0601.
[01:12:50] For more information about an error, try `rustc --explain E0425`.
---
[01:12:50] 
[01:12:50] 20 
[01:12:50] 21 error: aborting due to 2 previous errors
[01:12:50] 22 
[01:12:50] - Some errors occurred: E0223, E0599.
[01:12:50] + Some errors have detailed explanations: E0223, E0599.
[01:12:50] 25 
[01:12:50] 
[01:12:50] 
[01:12:50] The actual stderr differed from the expected stderr.
[01:12:50] The actual stderr differed from the expected stderr.
[01:12:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-std-when-using-type/suggest-std-when-using-type.stderr
[01:12:50] To update references, rerun the tests and pass the `--bless` flag
[01:12:50] To only update this specific test, also pass `--test-args suggestions/suggest-std-when-using-type.rs`
[01:12:50] error: 1 errors occurred comparing output.
[01:12:50] status: exit code: 1
[01:12:50] status: exit code: 1
[01:12:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-std-when-using-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-std-when-using-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-std-when-using-type/auxiliary" "-A" "unused"
[01:12:50] ------------------------------------------
[01:12:50] 
[01:12:50] ------------------------------------------
[01:12:50] stderr:
[01:12:50] stderr:
[01:12:50] ------------------------------------------
[01:12:50] error[E0223]: ambiguous associated type
[01:12:50]   --> /checkout/src/test/ui/suggestions/suggest-std-when-using-type.rs:2:14
[01:12:50]    |
[01:12:50] LL |     let pi = f32::consts::PI; //~ ERROR ambiguous associated type
[01:12:50] help: you are looking for the module in `std`, not the primitive type
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |     let pi = std::f32::consts::PI; //~ ERROR ambiguous associated type
[01:12:50] 
[01:12:50] error[E0599]: no function or associated item named `from_utf8` found for type `str` in the current scope
[01:12:50]   --> /checkout/src/test/ui/suggestions/suggest-std-when-using-type.rs:5:14
[01:12:50]    |
[01:12:50]    |
[01:12:50] LL |         str::from_utf8(bytes) //~ ERROR no function or associated item named `from_utf8` found
[01:12:50]    |              ^^^^^^^^^ function or associated item not found in `str`
[01:12:50] help: you are looking for the module in `std`, not the primitive type
[01:12:50]    |
[01:12:50] LL |         std::str::from_utf8(bytes) //~ ERROR no function or associated item named `from_utf8` found
[01:12:50] 
[01:12:50] error: aborting due to 2 previous errors
[01:12:50] 
[01:12:50] Some errors have detailed explanations: E0223, E0599.
---
[01:12:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:12:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:50] 
[01:12:50] 
[01:12:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:50] 
[01:12:50] 
[01:12:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:50] Build completed unsuccessfully in 0:04:46
[01:12:50] Build completed unsuccessfully in 0:04:46
[01:12:50] make: *** [check] Error 1
[01:12:50] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34da0534
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 19 01:42:53 UTC 2019
