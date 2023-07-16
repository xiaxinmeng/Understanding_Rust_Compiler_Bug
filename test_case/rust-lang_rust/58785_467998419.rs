plain
travis_time:end:180df758:start=1551290665431701518,finish=1551290878789953421,duration=213358251903
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:22] 
[01:22:22] running 119 tests
[01:22:45] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:22:49] i......iii.i.....ii
[01:22:49] 
[01:22:49]  finished in 27.445
[01:22:49] travis_fold:end:test_debuginfo

---
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:49] 
[01:22:49] running 19 tests
[01:22:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:22:55] ............FF.....
[01:22:55] 
[01:22:55] ---- [ui] ui-fulldeps/lint_tool_cmdline_allow.rs stdout ----
[01:22:55] 
[01:22:55] 
[01:22:55] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs" failed to compile: 
[01:22:55] status: exit code: 1
[01:22:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_cmdline_allow/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_cmdline_allow/auxiliary"
[01:22:55] ------------------------------------------
[01:22:55] 
[01:22:55] ------------------------------------------
[01:22:55] stderr:
[01:22:55] stderr:
[01:22:55] ------------------------------------------
[01:22:55] {"message":"attempted to repeat an expression containing no syntax variables matched as repeating at this depth","code":null,"level":"error","spans":[{"file_name":"<::rustc::lint::declare_tool_lint macros>","byte_start":140,"byte_end":154,"line_start":5,"line_end":5,"column_start":3,"column_end":17,"is_primary":true,"text":[{"text":"$ ( # [ attr ] ) * $ vis $ tool :: $ NAME , $ Level , $ desc , false } ) ; (","highlight_start":3,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attempted to repeat an expression containing no syntax variables matched as repeating at this depth\n  --> <::rustc::lint::declare_tool_lint macros>:5:3\n   |\nLL | $ ( # [ attr ] ) * $ vis $ tool :: $ NAME , $ Level , $ desc , false } ) ; (\n   |   ^^^^^^^^^^^^^^\n\n"}
[01:22:55] 
[01:22:55] ------------------------------------------
[01:22:55] 
[01:22:55] thread '[ui] ui-fulldeps/lint_tool_cmdline_allow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:22:55] thread '[ui] ui-fulldeps/lint_tool_cmdline_allow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:22:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:22:55] 
[01:22:55] ---- [ui] ui-fulldeps/lint_tool_test.rs stdout ----
[01:22:55] 
[01:22:55] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs" failed to compile: 
[01:22:55] status: exit code: 1
[01:22:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/auxiliary"
[01:22:55] ------------------------------------------
[01:22:55] 
[01:22:55] ------------------------------------------
[01:22:55] stderr:
[01:22:55] stderr:
[01:22:55] ------------------------------------------
[01:22:55] {"message":"attempted to repeat an expression containing no syntax variables matched as repeating at this depth","code":null,"level":"error","spans":[{"file_name":"<::rustc::lint::declare_tool_lint macros>","byte_start":140,"byte_end":154,"line_start":5,"line_end":5,"column_start":3,"column_end":17,"is_primary":true,"text":[{"text":"$ ( # [ attr ] ) * $ vis $ tool :: $ NAME , $ Level , $ desc , false } ) ; (","highlight_start":3,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attempted to repeat an expression containing no syntax variables matched as repeating at this depth\n  --> <::rustc::lint::declare_tool_lint macros>:5:3\n   |\nLL | $ ( # [ attr ] ) * $ vis $ tool :: $ NAME , $ Level , $ desc , false } ) ; (\n   |   ^^^^^^^^^^^^^^\n\n"}
[01:22:55] 
[01:22:55] ------------------------------------------
[01:22:55] 
[01:22:55] thread '[ui] ui-fulldeps/lint_tool_test.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:22:55] test result: FAILED. 17 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:22:55] 
[01:22:55] 
[01:22:55] 
[01:22:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:55] 
[01:22:55] 
[01:22:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:55] Build completed unsuccessfully in 0:11:45
[01:22:55] Build completed unsuccessfully in 0:11:45
[01:22:55] make: *** [check] Error 1
[01:22:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24234daa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 27 19:31:03 UTC 2019
---
travis_time:end:14adf157:start=1551295864926565349,finish=1551295864931009321,duration=4443972
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03e48130
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $C
