plain
[00:50:15] ....................................................................................................
[00:50:18] ............................................................i.......................................
[00:50:20] ....................................................................................................
[00:50:24] ....................................................................................................
[00:50:26] .........iiiiiiiii..................................................................................
[00:50:32] ....................................................................................................
[00:50:36] .............................................................................................i......
[00:50:38] ....................................................................................................
[00:50:41] .....................................................i.i..ii........................................
---
[00:57:05] 
[00:57:05] running 3089 tests
[00:57:15] ....................................................................................................
[00:57:26] .............................................................................i......................
[00:57:34] .............................................................F......................................
[00:57:52] ....................................................................................................
[00:58:02] ....................................................................................................
[00:58:16] ....................................................................................................
[00:58:26] ....................................................................................................
---
[00:59:21] ....................................................................................................
[00:59:32] ..................................................................................................F.
[00:59:43] .............i......................................................................................
[00:59:51] ....................................................................................................
[01:00:02] ......................................i..........F..................................................
[01:00:26] ....................................................................................................
[01:00:35] ......................................................................................i.............
[01:00:46] ........................................................i...........................................
[01:01:07] ....................................................................................................
---
[01:02:48] normalized stderr:
[01:02:48] warning: variable does not need to be mutable
[01:02:48]   --> $DIR/nested-matchs.rs:17:13
[01:02:48]    |
[01:02:48] LL |         let mut bar;
[01:02:48]    |             |
[01:02:48]    |             help: remove this `mut`
[01:02:48]    |
[01:02:48]    = note: #[warn(unused_mut)] on by default
[01:02:48]    = note: #[warn(unused_mut)] on by default
[01:02:48] 
[01:02:48] 
[01:02:48] 
[01:02:48] 
[01:02:48] The actual stderr differed from the expected stderr.
[01:02:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/nested-matchs.nll/nested-matchs.nll.stderr
[01:02:48] To update references, rerun the tests and pass the `--bless` flag
[01:02:48] To only update this specific test, also pass `--test-args binding/nested-matchs.rs`
[01:02:48] error: 1 errors occurred comparing output.
[01:02:48] status: exit code: 0
[01:02:48] status: exit code: 0
[01:02:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/binding/nested-matchs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/nested-matchs.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/nested-matchs.nll/auxiliary"
[01:02:48] ------------------------------------------
[01:02:48] 
[01:02:48] ------------------------------------------
[01:02:48] stderr:
[01:02:48] stderr:
[01:02:48] ------------------------------------------
[01:02:48] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/binding/nested-matchs.rs","byte_start":589,"byte_end":596,"line_start":17,"line_end":17,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"        let mut bar;","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_mut)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/binding/nested-matchs.rs","byte_start":589,"byte_end":593,"line_start":17,"line_end":17,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"        let mut bar;","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: variable does not need to be mutable\n  --> /checkout/src/test/run-pass/binding/nested-matchs.rs:17:13\n   |\nLL |         let mut bar;\n   |             ----^^^\n   |             |\n   |             help: remove this `mut`\n   |\n   = note: #[warn(unused_mut)] on by default\n\n"}
[01:02:48] ------------------------------------------
[01:02:48] 
[01:02:48] thread '[run-pass (nll)] run-pass/binding/nested-matchs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:02:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:48] 
[01:02:48] ---- [run-pass (nll)] run-pass/issues/issue-30371.rs stdout ----
[01:02:48] normalized stderr:
[01:02:48] warning: variable does not need to be mutable
[01:02:48]   --> $DIR/issue-30371.rs:16:14
[01:02:48]    |
[01:02:48] LL |       for _ in match return () {
[01:02:48]    |                ^-----
[01:02:48]    |  ______________help: remove this `mut`
[01:02:48]    | |
[01:02:48]    | |
[01:02:48] LL | |         () => Some(0),
[01:02:48] LL | |     } {}
[01:02:48]    |
[01:02:48]    = note: #[warn(unused_mut)] on by default
[01:02:48] 
[01:02:48] 
[01:02:48] 
[01:02:48] 
[01:02:48] 
[01:02:48] The actual stderr differed from the expected stderr.
[01:02:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-30371.nll/issue-30371.nll.stderr
[01:02:48] To update references, rerun the tests and pass the `--bless` flag
[01:02:48] To only update this specific test, also pass `--test-args issues/issue-30371.rs`
[01:02:48] error: 1 errors occurred comparing output.
[01:02:48] status: exit code: 0
[01:02:48] status: exit code: 0
[01:02:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-30371.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-30371.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-30371.nll/auxiliary"
[01:02:48] ------------------------------------------
[01:02:48] 
[01:02:48] ------------------------------------------
[01:02:48] stderr:
[01:02:48] stderr:
[01:02:48] ------------------------------------------
[01:02:48] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-30371.rs","byte_start":560,"byte_end":606,"line_start":16,"line_end":18,"column_start":14,"column_end":6,"is_primary":true,"text":[{"text":"    for _ in match return () {","highlight_start":14,"highlight_end":31},{"text":"        () => Some(0),","highlight_start":1,"highlight_end":23},{"text":"    } {}","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_mut)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-30371.rs","byte_start":560,"byte_end":566,"line_start":16,"line_end":16,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for _ in match return () {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: variable does not need to be mutable\n  --> /checkout/src/test/run-pass/issues/issue-30371.rs:16:14\n   |\nLL |       for _ in match return () {\n   |                ^-----\n   |                |\n   |  ______________help: remove this `mut`\n   | |\nLL | |         () => Some(0),\nLL | |     } {}\n   | |_____^\n   |\n   = note: #[warn(unused_mut)] on by default\n\n"}
[01:02:48] ------------------------------------------
[01:02:48] 
[01:02:48] thread '[run-pass (nll)] run-pass/issues/issue-30371.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:02:48] 
[01:02:48] 
[01:02:48] ---- [run-pass (nll)] run-pass/issues/issue-49298.rs stdout ----
[01:02:48] normalized stderr:
[01:02:48] warning: variable does not need to be mutable
[01:02:48]   --> $DIR/issue-49298.rs:19:9
[01:02:48]    |
[01:02:48] LL |     let mut x: (Void, usize);
[01:02:48]    |         |
[01:02:48]    |         help: remove this `mut`
[01:02:48]    |
[01:02:48]    = note: #[warn(unused_mut)] on by default
[01:02:48]    = note: #[warn(unused_mut)] on by default
[01:02:48] 
[01:02:48] 
[01:02:48] 
[01:02:48] 
[01:02:48] The actual stderr differed from the expected stderr.
[01:02:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-49298.nll/issue-49298.nll.stderr
[01:02:48] To update references, rerun the tests and pass the `--bless` flag
[01:02:48] To only update this specific test, also pass `--test-args issues/issue-49298.rs`
[01:02:48] error: 1 errors occurred comparing output.
[01:02:48] status: exit code: 0
[01:02:48] status: exit code: 0
[01:02:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-49298.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-49298.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pa:48] 
[01:02:48] 
[01:02:48] thread '[run-pass (nll)] run-pass/issues/issue-49298.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:02:48] 
[01:02:48] 
---
[01:02:48] 
[01:02:48] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:02:48] 
[01:02:48] 
[01:02:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:02:48] 
[01:02:48] 
[01:02:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:48] Build completed unsuccessfully in 0:16:44
[01:02:48] Build completed unsuccessfully in 0:16:44
[01:02:48] make: *** [check] Error 1
[01:02:48] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c8746f5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
