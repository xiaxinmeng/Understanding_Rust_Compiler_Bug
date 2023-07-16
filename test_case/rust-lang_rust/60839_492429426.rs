plain
travis_time:end:224a7520:start=1557867525699836932,finish=1557867528231357459,duration=2531520527
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:45] .................................................................................................... 300/5531
[01:09:48] .................................................................................................... 400/5531
[01:09:51] ..................................................................................................i. 500/5531
[01:09:55] .................................................................................................... 600/5531
[01:09:59] ....................................................................F............................... 700/5531
[01:10:09] ...............................................................................i...............i.... 900/5531
[01:10:13] .................................................................................................... 1000/5531
[01:10:16] ............iiiii................................................................................... 1100/5531
[01:10:19] .................................................................................................... 1200/5531
---
[01:13:01] failures:
[01:13:01] 
[01:13:01] ---- [ui] ui/const-generics/cannot-infer-type-for-const-param.rs stdout ----
[01:13:01] 
[01:13:01] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:13:01] status: exit code: 101
[01:13:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param/auxiliary" "-A" "unused"
[01:13:01] ------------------------------------------
[01:13:01] 
[01:13:01] ------------------------------------------
[01:13:01] stderr:
[01:13:01] stderr:
[01:13:01] ------------------------------------------
[01:13:01] warning: the feature `const_generics` is incomplete and may cause the compiler to crash
[01:13:01]   --> /checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs:1:12
[01:13:01]    |
[01:13:01] LL | #![feature(const_generics)]
[01:13:01]    |            ^^^^^^^^^^^^^^
[01:13:01] 
[01:13:01] error: internal compiler error: broken MIR in DefId(0:17 ~ cannot_infer_type_for_const_param[317d]::main[0]) (_1 = Foo::<_>(move _2,)): bad assignment (Foo<Const { ty: usize, val: Scalar(Bits { size: 8, bits: 3 }) }> = Foo<_>): NoSolution
[01:13:01]    |
[01:13:01]    |
[01:13:01] LL |     let _ = Foo::<3>([1, 2, 3]); //~ ERROR type annotations needed
[01:13:01] 
[01:13:01] 
[01:13:01] error: internal compiler error: broken MIR in DefId(0:17 ~ cannot_infer_type_for_const_param[317d]::main[0]) (_1 = Foo::<_>(move _2,)): bad user type on rvalue (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:12 ~ cannot_infer_type_for_const_param[317d]::Foo[0]), UserSubsts { substs: [Const { ty: usize, val: Unevaluated(DefId(0:18 ~ cannot_infer_type_for_const_param[317d]::main[0]::{{constant}}[0]), []) }], user_self_ty: None }) }, span: /checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs:10:13: 10:32, inferred_ty: Foo<Const { ty: usize, val: Scalar(Bits { size: 8, bits: 3 }) }> } = Foo<_>): NoSolution
[01:13:01]    |
[01:13:01]    |
[01:13:01] LL |     let _ = Foo::<3>([1, 2, 3]); //~ ERROR type annotations needed
[01:13:01] 
[01:13:01] 
[01:13:01] thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:356:17
[01:13:01] 
[01:13:01] error: internal compiler error: unexpected panic
[01:13:01] 
[01:13:01] note: the compiler unexpectedly panicked. this is a bug.
[01:13:01] note: the compiler unexpectedly panicked. this is a bug.
[01:13:01] 
[01:13:01] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:13:01] 
[01:13:01] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:13:01] 
[01:13:01] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:13:01] 
[01:13:01] ------------------------------------------
[01:13:01] 
[01:13:01] 
---
[01:13:01] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:13:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:01] 
[01:13:01] 
[01:13:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:01] 
[01:13:01] 
[01:13:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:01] Build completed unsuccessfully in 0:04:42
[01:13:01] Build completed unsuccessfully in 0:04:42
[01:13:01] make: *** [check] Error 1
[01:13:01] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0725b959
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 22:12:00 UTC 2019
