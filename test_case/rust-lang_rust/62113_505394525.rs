plain
travis_time:end:1f7aa6b0:start=1561456865130605559,finish=1561456866652339815,duration=1521734256
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
[00:54:19] 
[00:54:19] running 5723 tests
[00:54:24] .............................F...................................................................... 100/5723
[00:54:34] .................................................................................................... 300/5723
[00:54:37] .................................................................................................... 400/5723
[00:54:37] .................................................................................................... 400/5723
[00:54:40] .................................................................................................F.. 500/5723
[00:54:47] .................................................................................................... 700/5723
[00:54:51] .................................................................................................... 800/5723
[00:54:56] .................................................................................................... 900/5723
[00:54:56] .................................................................................................... 900/5723
[00:55:01] ..............................................................i.....F.....i......................... 1000/5723
[00:55:04] ...........................................................................................iiiii.... 1100/5723
[00:55:10] .................................................................................................... 1300/5723
[00:55:13] .................................................................................................... 1400/5723
[00:55:16] .................................................................................................... 1500/5723
[00:55:19] .................................................................................................... 1600/5723
---
[00:56:06] .................................................................................................... 2900/5723
[00:56:09] .................................................................................................... 3000/5723
[00:56:14] .................................................................................................... 3100/5723
[00:56:17] .................................................................................................... 3200/5723
[00:56:20] ..................................................F................................................. 3300/5723
[00:56:28] ......................i............................................................................. 3500/5723
[00:56:32] .................................................................................................ii. 3600/5723
[00:56:35] ..i..ii............................................................................................. 3700/5723
[00:56:39] .................................................................................................... 3800/5723
---
[00:57:57] failures:
[00:57:57] 
[00:57:57] ---- [ui] ui/asm/asm-src-loc-codegen-units.rs stdout ----
[00:57:57] normalized stderr:
[00:57:57] error: <inline asm>:1:2: error: invalid instruction mnemonic 'nowayisthisavalidinstruction'
[00:57:57]         nowayisthisavalidinstruction
[00:57:57] 
[00:57:57]   --> $DIR/asm-src-loc-codegen-units.rs:10:9
[00:57:57]    |
[00:57:57]    |
[00:57:57] LL |         asm!("nowayisthisavalidinstruction");
[00:57:57] 
[00:57:57] error: aborting due to previous error
[00:57:57] 
[00:57:57] 
[00:57:57] 
[00:57:57] 
[00:57:57] 
[00:57:57] The actual stderr differed from the expected stderr.
[00:57:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/asm-src-loc-codegen-units/asm-src-loc-codegen-units.stderr
[00:57:57] To update references, rerun the tests and pass the `--bless` flag
[00:57:57] To only update this specific test, also pass `--test-args asm/asm-src-loc-codegen-units.rs`
[00:57:57] error: 1 errors occurred comparing output.
[00:57:57] status: exit code: 1
[00:57:57] status: exit code: 1
[00:57:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/asm-src-loc-codegen-units.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/asm-src-loc-codegen-units" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "codegen-units=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/asm-src-loc-codegen-units/auxiliary" "-A" "unused"
[00:57:57] ------------------------------------------
[00:57:57] 
[00:57:57] ------------------------------------------
[00:57:57] stderr:
[00:57:57] stderr:
[00:57:57] ------------------------------------------
[00:57:57] error: <inline asm>:1:2: error: invalid instruction mnemonic 'nowayisthisavalidinstruction'
[00:57:57]         nowayisthisavalidinstruction
[00:57:57] 
[00:57:57]   --> /checkout/src/test/ui/asm/asm-src-loc-codegen-units.rs:10:9
[00:57:57]    |
[00:57:57]    |
[00:57:57] LL |         asm!("nowayisthisavalidinstruction"); //~ ERROR instruction
[00:57:57] 
[00:57:57] error: aborting due to previous error
[00:57:57] 
[00:57:57] 
[00:57:57] 
[00:57:57] ------------------------------------------
[00:57:57] 
[00:57:57] 
[00:57:57] ---- [ui] ui/chalkify/recursive_where_clause_on_type.rs stdout ----
[00:57:57] diff of stderr:
[00:57:57] 
[00:57:57] 1 error: the type `S` is not well-formed (chalk)
[00:57:57] +   --> $DIR/recursive_where_clause_on_type.rs:26:11
[00:57:57] 3    |
[00:57:57] 3    |
[00:57:57] 4 LL |     foo::<S>()
[00:57:57] +    |           ^
[00:57:57] 6 
[00:57:57] 6 
[00:57:57] 7 error: the type `S` is not well-formed (chalk)
[00:57:57] +   --> $DIR/recursive_where_clause_on_type.rs:26:5
[00:57:57] 9    |
[00:57:57] 9    |
[00:57:57] 10 LL |     foo::<S>()
[00:57:57] +    |     ^^^^^^^^
[00:57:57] 12 
[00:57:57] 13 error: aborting due to 2 previous errors
[00:57:57] 14 
[00:57:57] 14 
[00:57:57] 
[00:57:57] 
[00:57:57] The actual stderr differed from the expected stderr.
[00:57:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/recursive_where_clause_on_type/recursive_where_clause_on_type.stderr
[00:57:57] To update references, rerun the tests and pass the `--bless` flag
[00:57:57] To only update this specific test, also pass `--test-args chalkify/recursive_where_clause_on_type.rs`
[00:57:57] error: 1 errors occurred comparing output.
[00:57:57] status: exit code: 1
[00:57:57] status: exit code: 1
[00:57:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/recursive_where_clause_on_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/recursive_where_clause_on_type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "chalk" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/recursive_where_clause_on_type/auxiliary" "-A" "unused"
[00:57:57] ------------------------------------------
[00:57:57] 
[00:57:57] ------------------------------------------
[00:57:57] stderr:
[00:57:57] stderr:
[00:57:57] ------------------------------------------
[00:57:57] error: the type `S` is not well-formed (chalk)
[00:57:57]    |
[00:57:57]    |
[00:57:57] LL |     foo::<S>() //~ ERROR the type `S` is not well-formed (chalk)
[00:57:57] 
[00:57:57] 
[00:57:57] error: the type `S` is not well-formed (chalk)
[00:57:57]    |
[00:57:57]    |
[00:57:57] LL |     foo::<S>() //~ ERROR the type `S` is not well-formed (chalk)
[00:57:57] 
[00:57:57] error: aborting due to 2 previous errors
[00:57:57] 
[00:57:57] 
---
[00:57:57] 
[00:57:57] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:57:57] 6    |
[00:57:57] 7    = note: candidates:
[00:57:57] -            crate `crateresolve1`: $TEST_BUILD_DIR/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-3.so
[00:57:57] -            crate `crateresolve1`: $TEST_BUILD_DIR/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-1.so
[00:57:57] 10            crate `crateresolve1`: $TEST_BUILD_DIR/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-2.so
[00:57:57] +            crate `crateresolve1`: $TEST_BUILD_DIR/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-1.so
[00:57:57] +            crate `crateresolve1`: $TEST_BUILD_DIR/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-3.so
[00:57:57] 12 error[E0463]: can't find crate for `crateresolve1`
[00:57:57] 13   --> $DIR/crateresolve1.rs:6:1
[00:57:57] 
[00:57:57] 
[00:57:57] 
[00:57:57] The actual stderr differed from the expected stderr.
[00:57:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crateresolve1/crateresolve1/crateresolve1.stderr
[00:57:57] To update references, rerun the tests and pass the `--bless` flag
[00:57:57] To only update this specific test, also pass `--test-args crateresolve1/crateresolve1.rs`
[00:57:57] error: 1 errors occurred comparing output.
[00:57:57] status: exit code: 1
[00:57:57] status: exit code: 1
[00:57:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crateresolve1/crateresolve1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crateresolve1/crateresolve1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crateresolve1/crateresolve1/auxiliary" "-A" "unused"
[00:57:57] ------------------------------------------
[00:57:57] 
[00:57:57] ------------------------------------------
[00:57:57] stderr:
---
[00:57:57] LL | extern crate crateresolve1;
[00:57:57]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:57:57]    |
[00:57:57]    = note: candidates:
[00:57:57]            crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-2.so
[00:57:57]            crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-1.so
[00:57:57]            crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crateresolve1/crateresolve1/auxiliary/libcrateresolve1-3.so
[00:57:57] error[E0463]: can't find crate for `crateresolve1`
[00:57:57]   --> /checkout/src/test/ui/crateresolve1/crateresolve1.rs:6:1
[00:57:57]    |
[00:57:57] LL | extern crate crateresolve1;
---
[00:57:57] diff of stderr:
[00:57:57] 
[00:57:57] 1 error: linking with `ld` failed: exit code: 1
[00:57:57] 2    |
[00:57:57] -    = note: "ld" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "$TEST_BUILD_DIR/linkage/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.0.rcgu.o" "$TEST_BUILD_DIR/linkage/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.1.rcgu.o" "-o" "$TEST_BUILD_DIR/linkage/invalid-link-args/invalid-link-args" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "$TEST_BUILD_DIR/linkage/invalid-link-args/auxiliary" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-df58114e777eef4d" "--end-group" "-Bstatic" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-34e2d1be8ef7ab6e.rlib" "-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,$LIB_DIR/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
[00:57:57] -    = note: ld: unrecognised option '-Wl,-rpath,$ORIGIN/../../../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib'
[00:57:57] +    = note: "ld" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "$TEST_BUILD_DIR/linkage/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.0.rcgu.o" "$TEST_BUILD_DIR/linkage/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.1.rcgu.o" "-o" "$TEST_BUILD_DIR/linkage/invalid-link-args/invalid-link-args" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "$TEST_BUILD_DIR/linkage/invalid-link-args/auxiliary" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-2e753a4979f4d43c" "--end-group" "-Bstatic" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-84bb07d1eb68f25a.rlib" "-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,$LIB_DIR/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
[00:57:57] +    = note: ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'
[00:57:57] 5            ld: use the --help option for usage information
[00:57:57] 7 
[00:57:57] 
[00:57:57] 
[00:57:57] The actual stderr differed from the expected stderr.
[00:57:57] The actual stderr differed from the expected stderr.
[00:57:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage/invalid-link-args/invalid-link-args.stderr
[00:57:57] To update references, rerun the tests and pass the `--bless` flag
[00:57:57] To only update this specific test, also pass `--test-args linkage/invalid-link-args.rs`
[00:57:57] error: 1 errors occurred comparing output.
[00:57:57] status: exit code: 1
[00:57:57] status: exit code: 1
[00:57:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage/invalid-link-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage/invalid-link-args" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "linker-flavor=ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage/invalid-link-args/auxiliary" "-A" "unused"
[00:57:57] ------------------------------------------
[00:57:57] 
[00:57:57] ------------------------------------------
[00:57:57] stderr:
[00:57:57] stderr:
[00:57:57] ------------------------------------------
[00:57:57] error: linking with `ld` failed: exit code: 1
[00:57:57]    |
[00:57:57]    = note: "ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage/invalid-link-args/invalid-link-args" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage/invalid-link-args/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-2e753a4979f4d43c" "--end-group" "-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-84bb07d1eb68f25a.rlib" "-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
[00:57:57]    = note: ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'
[00:57:57]            ld: use the --help option for usage information
[00:57:57] 
[00:57:57] error: aborting due to previous error
[00:57:57] 
[00:57:57] 
---
[00:57:57] 
[00:57:57] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:57:57] 
[00:57:57] 
[00:57:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:57] 
[00:57:57] 
[00:57:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:57] Build completed unsuccessfully in 0:53:20
---
travis_time:end:1e35442f:start=1561460355508993106,finish=1561460355513955900,duration=4962794
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00decb04
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; 
