plain
[00:42:43]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:42:43]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:43:30] warning: [1] cannot be resolved, ignoring it...
[00:43:30] 
[00:43:31] warning: [oom] cannot be resolved, ignoring it...
[00:43:31] warning: [x] cannot be resolved, ignoring it...
[00:43:31] 
[00:43:31] warning: [] cannot be resolved, ignoring it...
[00:43:31] 
---
[00:43:40]     Checking unwind v0.0.0 (file:///checkout/src/libunwind)
[00:43:40]     Checking alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:43:40]     Checking panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:43:40]     Checking alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:43:44] warning: [std::alloc::set_oom_hook] cannot be resolved, ignoring it...
[00:43:44] 
[00:43:44] warning: [std::alloc::take_oom_hook] cannot be resolved, ignoring it...
[00:43:44]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:43:44]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:43:44]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:43:44]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:43:44]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:43:44]     Checking rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:43:44]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:43:55] warning: [oom] cannot be resolved, ignoring it...
[00:43:55] 
[00:43:57] warning: [System] cannot be resolved, ignoring it...
[00:43:57] 
[00:43:57] warning: [GlobalAlloc] cannot be resolved, ignoring it...
[00:44:02]     Finished release [optimized] target(s) in 1m 19.61s
[00:44:02] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:44:03]     Checking getopts v0.2.17
[00:44:03]     Checking term v0.0.0 (file:///checkout/src/libterm)
[00:44:03]     Checking term v0.0.0 (file:///checkout/src/libterm)
[00:44:03]  Documenting test v0.0.0 (file:///checkout/src/libtest)
[00:44:04] warning: [System] cannot be resolved, ignoring it...
[00:44:04] 
[00:44:04] warning: [GlobalAlloc] cannot be resolved, ignoring it...
[00:44:04]     Finished release [optimized] target(s) in 2.07s
[00:44:05] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[00:44:05]     Checking nodrop v0.1.12
[00:44:05]     Checking cfg-if v0.1.2
---
[00:44:15]     Checking syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:44:17]     Checking rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:44:18]     Checking syntax v0.0.0 (file:///checkout/src/libsyntax)
[00:44:35]  Documenting proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:44:36] warning: [System] cannot be resolved, ignoring it...
[00:44:36] 
[00:44:36] warning: [GlobalAlloc] cannot be resolved, ignoring it...
[00:44:36] warning: [cfg] cannot be resolved, ignoring it...
[00:44:36] 
[00:44:36] warning: [rayon::prelude] cannot be resolved, ignoring it...
[00:44:36] 
---
[00:47:02] ......................................................................i.............................
[00:47:06] ....................................................................................................
[00:47:12] ....................................................................................................
[00:47:18] ....................................................................................................
[00:47:22] ..i.................iiiiiiiii...................................................
[00:47:22] 
[00:47:22] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:10] ......................................................................i.............................
[00:48:14] ....................................................................................................
[00:48:19] ....................................................................................................
[00:48:25] ....................................................................................................
[00:48:29] ..i..................iiiiiiiii..................................................
[00:48:29] 
[00:48:29]  finished in 66.965
[00:48:29] travis_fold:end:test_ui_nll

---
[01:29:30] 
[01:29:30] ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
[01:29:30] diff of stderr:
[01:29:30] 
[01:29:30] + warning: [System] cannot be resolved, ignoring it...
[01:29:30] + 
[01:29:30] + warning: [GlobalAlloc] cannot be resolved, ignoring it...
[01:29:30] + 
[01:29:30] 1 warning: [Foo::baz] cannot be resolved, ignoring it...
[01:29:30] 2 
[01:29:30] 3 warning: [Bar::foo] cannot be resolved, ignoring it...
[01:29:30] 
[01:29:30] The actual stderr differed from the expected stderr.
[01:29:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
[01:29:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
[01:29:30] To update references, rerun the tests and pass the `--bless` flag
[01:29:30] To only update this specific test, also pass `--test-args intra-links-warning.rs`
[01:29:30] error: 1 errors occurred comparing output.
[01:29:30] status: exit code: 0
[01:29:30] status: exit code: 0
[01:29:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxiliary"
[01:29:30] ------------------------------------------
[01:29:30] 
[01:29:30] ------------------------------------------
[01:29:30] stderr:
[01:29:30] stderr:
[01:29:30] ------------------------------------------
[01:29:30] {"message":"[System] cannot be resolved, ignoring it...","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: [System] cannot be resolved, ignoring it...\n\n"}
[01:29:30] {"message":"[GlobalAlloc] cannot be resolved, ignoring it...","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: [GlobalAlloc] cannot be resolved, ignoring it...\n\n"}
[01:29:30] {"message":"[Foo::baz] cannot be resolved, ignoring it...","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: [Foo::baz] cannot be resolved, ignoring it...\n\n"}
[01:29:30] {"message":"[Bar::foo] cannot be resolved, ignoring it...","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: [Bar::foo] cannot be resolved, ignoring it...\n\n"}
[01:29:30] {"message":"[Uniooon::X] cannot be resolved, ignoring it...","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: [Uniooon::X] cannot be resolved, ignoring it...\n\n"}
[01:29:30] ------------------------------------------
[01:29:30] 
[01:29:30] thread '[ui] rustdoc-ui/intra-links-warning.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:29:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:29:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:29:30] 
[01:29:30] ---- [ui] rustdoc-ui/deprecated-attrs.rs stdout ----
[01:29:30] diff of stderr:
[01:29:30] 
[01:29:30] + warning: [System] cannot be resolved, ignoring it...
[01:29:30] + 
[01:29:30] + warning: [GlobalAlloc] cannot be resolved, ignoring it...
[01:29:30] + 
[01:29:30] 1 warning: the `#![doc(no_default_passes)]` attribute is considered deprecated
[01:29:30] 3   = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:29:30] 
[01:29:30] 
[01:29:30] The actual stderr differed from the expected stderr.
[01:29:30] The actual stderr differed from the expected stderr.
[01:29:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/deprecated-attrs.stderr
[01:29:30] To update references, rerun the tests and pass the `--bless` flag
[01:29:30] To only update this specific test, also pass `--test-args deprecated-attrs.rs`
[01:29:30] error: 1 errors occurred comparing output.
[01:29:30] status: exit code: 0
[01:29:30] status: exit code: 0
[01:29:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deprecated-attrs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/auxiliary"
[01:29:30] ------------------------------------------
[01:29:30] 
[01:29:30] ------------------------------------------
[01:29:30] stderr:
[01:29:30] stderr:
[01:29:30] ------------------------------------------
[01:29:30] {"message":"[System] cannot be resolved, ignoring it...","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: [System] cannot be resolved, ignoring it...\n\n"}
[01:29:30] {"message":"[GlobalAlloc] cannot be resolved, ignoring it...","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: [GlobalAlloc] cannot be resolved, ignoring it...\n\n"}
[01:29:30] {"message":"the `#![doc(no_default_passes)]` attribute is considered deprecated","code":null,"level":"warning","spans":[],"children":[{"message":"please see https://github.com/rust-lang/rust/issues/44136","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"you may want to use `#![doc(document_private_items)]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: the `#![doc(no_default_passes)]` attribute is considered deprecated\n  |\n  = warning: please see https://github.com/rust-lang/rust/issues/44136\n  = help: you may want to use `#![doc(document_private_items)]`\n\n"}
[01:29:30] {"message":"the `#![doc(passes = \"...\")]` attribute is considered deprecated","code":null,"level":"warning","spans":[],"children":[{"message":"please see https://github.com/rust-lang/rust/issues/44136","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning: the `#![doc(passes = \"...\")]` attribute is considered deprecated\n  |\n  = warning: please see https://github.com/rust-lang/rust/issues/44136\n\n"}
[01:29:30] ------------------------------------------
[01:29:30] 
[01:29:30] thread '[ui] rustdoc-ui/deprecated-attrs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:29:30] 
---
[01:29:30] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:29:30] 
[01:29:30] 
[01:29:30] 
[01:29:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:29:30] 
[01:29:30] 
[01:29:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:30] Build completed unsuccessfully in 0:44:46
[01:29:30] Build completed unsuccessfully in 0:44:46
[01:29:30] make: *** [check] Error 1
[01:29:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11e8453e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
