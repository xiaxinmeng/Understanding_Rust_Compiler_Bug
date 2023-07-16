plain
[00:47:00] .................................................................................................... 1400/4605
[00:47:02] .........................................................i.......................................... 1500/4605
[00:47:06] ........................i........................................................................... 1600/4605
[00:47:09] .................................................................................................... 1700/4605
[00:47:12] ..................................................F................................................. 1800/4605
[00:47:19] .................................................................................................... 2000/4605
[00:47:23] .................................................................................................... 2100/4605
[00:47:27] .................................................................................................... 2200/4605
[00:47:31] .................................................................................................... 2300/4605
---
[00:48:50] ---- [ui] ui/issues/issue-11692-2.rs stdout ----
[00:48:50] diff of stderr:
[00:48:50] 
[00:48:50] 3    |
[00:48:50] 4 LL |     concat!(test!()); //~ ERROR cannot find macro `test!` in this scope
[00:48:50] +    |
[00:48:50] +    |
[00:48:50] +    = help: have you added the `#[macro_use]` on the module/import?
[00:48:50] 7 error: aborting due to previous error
[00:48:50] 8 
[00:48:50] 
[00:48:50] 
[00:48:50] 
[00:48:50] The actual stderr differed from the expected stderr.
[00:48:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11692-2/issue-11692-2.stderr
[00:48:50] To update references, rerun the tests and pass the `--bless` flag
[00:48:50] To only update this specific test, also pass `--test-args issues/issue-11692-2.rs`
[00:48:50] error: 1 errors occurred comparing output.
[00:48:50] status: exit code: 1
[00:48:50] status: exit code: 1
[00:48:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-11692-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11692-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11692-2/auxiliary" "-A" "unused"
[00:48:50] ------------------------------------------
[00:48:50] 
[00:48:50] ------------------------------------------
[00:48:50] stderr:
[00:48:50] stderr:
[00:48:50] ------------------------------------------
[00:48:50] {"message":"cannot find macro `test!` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-11692-2.rs","byte_start":491,"byte_end":495,"line_start":12,"line_end":12,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    concat!(test!()); //~ ERROR cannot find macro `test!` in this scope","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"have you added the `#[macro_use]` on the module/import?","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `test!` in this scope\n  --> /checkout/src/test/ui/issues/issue-11692-2.rs:12:13\n   |\nLL |     concat!(test!()); //~ ERROR cannot find macro `test!` in this scope\n   |             ^^^^\n   |\n   = help: have you added the `#[macro_use]` on the module/import?\n\n"}
[00:48:50] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:50] ------------------------------------------
[00:48:50] 
[00:48:50] thread '[ui] ui/issues/issue-11692-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:48:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:50] 
[00:48:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:48:50] 
[00:48:50] 
[00:48:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:50] 
[00:48:50] 
[00:48:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:50] Build completed unsuccessfully in 0:03:33
[00:48:50] Build completed unsuccessfully in 0:03:33
[00:48:50] Makefile:58: recipe for target 'check' failed
[00:48:50] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001b5a5a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
