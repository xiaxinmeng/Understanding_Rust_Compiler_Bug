plain
travis_time:end:2c19a185:start=1558389884168661708,finish=1558389989483240960,duration=105314579252
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:47] .................................................................................................... 300/5536
[01:09:50] .................................................................................................... 400/5536
[01:09:53] .................................................................................................i.. 500/5536
[01:09:57] .................................................................................................... 600/5536
[01:10:01] .....................................................................F.............................. 700/5536
[01:10:11] ...................................................................................i...............i 900/5536
[01:10:15] .................................................................................................... 1000/5536
[01:10:19] ................iiiii............................................................................... 1100/5536
[01:10:21] .................................................................................................... 1200/5536
---
[01:13:03] 
[01:13:03] ---- [ui] ui/const-generics/cannot-infer-type-for-const-param.rs stdout ----
[01:13:03] diff of stderr:
[01:13:03] 
[01:13:03] 10 LL |     let _ = Foo::<3>([1, 2, 3]);
[01:13:03] 11    |                   ^ cannot infer type for `{integer}`
[01:13:03] - error: aborting due to previous error
[01:13:03] + error[E0308]: mismatched types
[01:13:03] +   --> $DIR/cannot-infer-type-for-const-param.rs:10:22
[01:13:03] +    |
[01:13:03] +    |
[01:13:03] + LL |     let _ = Foo::<3>([1, 2, 3]);
[01:13:03] +    |                      ^^^^^^^^^ expected `Const { ty: usize, val: Unevaluated(DefId(0:18 ~ cannot_infer_type_for_const_param[317d]::main[0]::{{constant}}[0]), []) }`, found `Const { ty: usize, val: Scalar(Bits { size: 8, bits: 3 }) }`
[01:13:03] +    |
[01:13:03] +    = note: expected type `[u8; _]`
[01:13:03] +               found type `[u8; 3]`
[01:13:03] - For more information about this error, try `rustc --explain E0282`.
[01:13:03] + error: aborting due to 2 previous errors
[01:13:03] + 
[01:13:03] + Some errors have detailed explanations: E0282, E0308.
[01:13:03] + Some errors have detailed explanations: E0282, E0308.
[01:13:03] + For more information about an error, try `rustc --explain E0282`.
[01:13:03] 16 
[01:13:03] 
[01:13:03] 
[01:13:03] The actual stderr differed from the expected stderr.
[01:13:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param/cannot-infer-type-for-const-param.stderr
[01:13:03] To update references, rerun the tests and pass the `--bless` flag
[01:13:03] To only update this specific test, also pass `--test-args const-generics/cannot-infer-type-for-const-param.rs`
[01:13:03] error: 1 errors occurred comparing output.
[01:13:03] status: exit code: 1
[01:13:03] status: exit code: 1
[01:13:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param/auxiliary" "-A" "unused"
[01:13:03] ------------------------------------------
[01:13:03] 
[01:13:03] ------------------------------------------
[01:13:03] stderr:
---
[01:13:03] 
[01:13:03] error[E0282]: type annotations needed
[01:13:03]   --> /checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs:10:19
[01:13:03]    |
[01:13:03] LL |     let _ = Foo::<3>([1, 2, 3]); //~ ERROR type annotations needed
[01:13:03]    |                   ^ cannot infer type for `{integer}`
[01:13:03] error[E0308]: mismatched types
[01:13:03]   --> /checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs:10:22
[01:13:03]    |
[01:13:03]    |
[01:13:03] LL |     let _ = Foo::<3>([1, 2, 3]); //~ ERROR type annotations needed
[01:13:03]    |                      ^^^^^^^^^ expected `Const { ty: usize, val: Unevaluated(DefId(0:18 ~ cannot_infer_type_for_const_param[317d]::main[0]::{{constant}}[0]), []) }`, found `Const { ty: usize, val: Scalar(Bits { size: 8, bits: 3 }) }`
[01:13:03]    |
[01:13:03]    = note: expected type `[u8; _]`
[01:13:03]               found type `[u8; 3]`
[01:13:03] error: aborting due to 2 previous errors
[01:13:03] 
[01:13:03] Some errors have detailed explanations: E0282, E0308.
[01:13:03] For more information about an error, try `rustc --explain E0282`.
---
[01:13:03] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:13:03] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:03] 
[01:13:03] 
[01:13:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:03] 
[01:13:03] 
[01:13:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:03] Build completed unsuccessfully in 0:04:43
[01:13:03] Build completed unsuccessfully in 0:04:43
[01:13:03] make: *** [check] Error 1
[01:13:03] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02163b5c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 20 23:19:41 UTC 2019
