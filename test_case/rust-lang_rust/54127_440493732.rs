plain
travis_time:end:051fb818:start=1542760089274074853,finish=1542760142405269747,duration=53131194894
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:18] .................................................................................................... 100/5043
[00:48:21] .................................................................................................... 200/5043
[00:48:24] ..............................ii...........................................ii...................ii.. 300/5043
[00:48:26] ..............................................................................................iii... 400/5043
[00:48:29] .....iiiiiiii.iii............................iii...........................................i........ 500/5043
[00:48:35] .................................................................................................... 700/5043
[00:48:41] ..................................................................................i...........i..... 800/5043
[00:48:44] .................................................................................................... 900/5043
[00:48:44] .................................................................................................... 900/5043
[00:48:47] .iiiii...................iiiiii..................................................................... 1000/5043
[00:48:49] ............................................................................iiiiiiii................ 1100/5043
[00:48:53] .................................................................................................... 1300/5043
[00:48:55] .................................................................................................... 1400/5043
[00:48:58] .................................i.................................................................. 1500/5043
[00:49:00] ..i.........ii.........................................................i............................ 1600/5043
---
[00:49:20] .................................................................................................... 2200/5043
[00:49:24] .................................................................................................... 2300/5043
[00:49:28] .................................................................................................... 2400/5043
[00:49:31] .................................................................................................... 2500/5043
[00:49:34] .....................................................................................iiiiiiiii...... 2600/5043
[00:49:41] ...................................................ii............................................... 2800/5043
[00:49:43] .................................................................................................... 2900/5043
[00:49:47] .................................................................................................... 3000/5043
[00:49:50] ...............................................i.................................................... 3100/5043
---
[00:52:25] .................................................................................................... 1000/2886
[00:52:37] .................................................................................................... 1100/2886
[00:52:44] .................................................................................................... 1200/2886
[00:52:53] .................................................................................................... 1300/2886
[00:53:04] .................................................................F............i..................... 1400/2886
[00:53:15] .................................................................................................... 1500/2886
[00:53:27] ......................................F........i.................................................... 1600/2886
[00:53:51] .................................................................................................... 1800/2886
[00:54:02] ..........................................................................i......................... 1900/2886
[00:54:18] .............................................i...................................................... 2000/2886
[00:54:43] .................................................................................................... 2100/2886
---
[00:56:26] normalized stderr:
[00:56:26] warning: unreachable expression
[00:56:26]   --> $DIR/issue-3037.rs:19:5
[00:56:26]    |
[00:56:26] LL |     match x {}
[00:56:26]    |
[00:56:26]    = note: #[warn(unreachable_code)] on by default
[00:56:26] 
[00:56:26] warning: unused variable: `x`
[00:56:26] warning: unused variable: `x`
[00:56:26]   --> $DIR/issue-3037.rs:18:9
[00:56:26]    |
[00:56:26] LL |     let x: Void = unimplemented!();
[00:56:26]    |         ^ help: consider using `_x` instead
[00:56:26]    = note: #[warn(unused_variables)] on by default
[00:56:26] 
[00:56:26] 
[00:56:26] 
[00:56:26] 
[00:56:26] 
[00:56:26] The actual stderr differed from the expected stderr.
[00:56:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3037/issue-3037.stderr
[00:56:26] To update references, rerun the tests and pass the `--bless` flag
[00:56:26] To only update this specific test, also pass `--test-args issues/issue-3037.rs`
[00:56:26] error: 1 errors occurred comparing output.
[00:56:26] status: exit code: 0
[00:56:26] status: exit code: 0
[00:56:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-3037.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3037/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3037/auxiliary"
[00:56:26] ------------------------------------------
[00:56:26] 
[00:56:26] ------------------------------------------
[00:56:26] stderr:
[00:56:26] stderr:
[00:56:26] ------------------------------------------
[00:56:26] {"message":"unreachable expression","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-3037.rs","byte_start":619,"byte_end":629,"line_start":19,"line_end":19,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    match x {}","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unreachable expression\n  --> /checkout/src/test/run-pass/issues/issue-3037.rs:19:5\n   |\nLL |     match x {}\n   |     ^^^^^^^^^^\n   |\n   = note: #[warn(unreachable_code)] on by default\n\n"}
[00:56:26] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-3037.rs","byte_start":587,"byte_end":588,"line_start":18,"line_end":18,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: Void = unimplemented!();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-3037.rs","byte_start":587,"byte_end":588,"line_start":18,"line_end":18,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: Void = unimplemented!();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/issues/issue-3037.rs:18:9\n   |\nLL |     let x: Void = unimplemented!();\n   |         ^ help: consider using `_x` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:56:26] ------------------------------------------
[00:56:26] 
[00:56:26] thread '[run-pass] run-pass/issues/issue-3037.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:56:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:26] 
[00:56:26] ---- [run-pass] run-pass/issues/issue-46855.rs stdout ----
[00:56:26] normalized stderr:
[00:56:26] warning: functions with parameters of uninhabited types are uncallable
[00:56:26]    |
[00:56:26]    |
[00:56:26] LL | fn foo(xs: [(Never, u32); 1]) -> u32 { xs[0].1 }
[00:56:26]    | ^^^^^^^--^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:56:26]    |        this parameter has an uninhabited type
[00:56:26]    |
[00:56:26]    = note: #[warn(unreachable_code)] on by default
[00:56:26] 
[00:56:26] 
[00:56:26] warning: functions with parameters of uninhabited types are uncallable
[00:56:26]    |
[00:56:26]    |
[00:56:26] LL | fn bar([(_, x)]: [(Never, u32); 1]) -> u32 { x }
[00:56:26]    |        |
[00:56:26]    |        this parameter has an uninhabited type
[00:56:26] 
[00:56:26] 
[00:56:26] 
[00:56:26] 
[00:56:26] 
[00:56:26] The actual stderr differed from the expected stderr.
[00:56:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gity":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/issues/issue-46855.rs","byte_start":630,"byte_end":678,"line_start":25,"line_end":25,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"fn foo(xs: [(Never, u32); 1]) -> u32 { xs[0].1 }","highlight_start":1,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/run-pass/issues/issue-46855.rs:25:1\n   |\nLL | fn foo(xs: [(Never, u32); 1]) -> u32 { xs[0].1 }\n   | ^^^^^^^--^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |        |\n   |        this parameter has an uninhabited type\n   |\n   = note: #[warn(unreachable_code)] on by default\n\n"}
[00:56:26] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-46855.rs","byte_start":687,"byte_end":695,"line_start":27,"line_end":27,"column_start":8,"column_end":16,"is_primary":false,"text":[{"text":"fn bar([(_, x)]: [(Never, u32); 1]) -> u32 { x }","highlight_start":8,"highlight_end":16}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/issues/issue-46855.rs","byte_start":680,"byte_end":728,"line_start":27,"line_end":27,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"fn bar([(_, x)]: [(Never, u32); 1]) -> u32 { x }","highlight_start":1,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/run-pass/issues/issue-46855.rs:27:1\n   |\nLL | fn bar([(_, x)]: [(Never, u32); 1]) -> u32 { x }\n   | ^^^^^^^--------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |        |\n   |        this parameter has an uninhabited type\n\n"}
[00:56:26] ------------------------------------------
[00:56:26] 
[00:56:26] thread '[run-pass] run-pass/issues/issue-46855.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:56:26] 
---
[00:56:26] 
[00:56:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:56:26] 
[00:56:26] 
[00:56:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:26] 
[00:56:26] 
[00:56:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:26] Build completed unsuccessfully in 0:11:44
[00:56:26] Build completed unsuccessfully in 0:11:44
[00:56:26] make: *** [check] Error 1
[00:56:26] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ea617f2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 01:25:37 UTC 2018
---
142668 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
136912 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
136908 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
134652 ./obj/build/bootstrap/debug/incremental/bootstrap-1crps3s9gh6vp
134648 ./obj/build/bootstrap/debug/incremental/bootstrap-1crps3s9gh6vp/s-f6v2ajvuge-xxqahu-1hoekcm7rhrrx
130772 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130768 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123692 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
