plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:56] 
[00:44:56] running 1400 tests
[00:45:00] ..................................................................................i.................
[00:45:06] ................................i..............................................................F....
[00:45:13] ....................................................................................................
[00:45:17] ....................................................................................................
[00:45:20] ....................................................................................................
[00:45:26] ....................................................................................................
[00:45:26] ....................................................................................................
[00:45:32] ....................................................................................................
[00:45:37] ....................................................................................................
[00:45:43] .....................................i..............................................................
[00:45:49] .............i......................................................................................
[00:45:54] ................................ii..................................................................
[00:46:01] ....................................................................................................
ormat" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-lint-paths.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-lint-paths.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] ------------------------------------------
[00:46:07] stderr:
[00:46:07] stderr:
[00:46:07] ------------------------------------------
[00:46:07] {"message":"unknown lint: `absolute_path_starting_with_module`","code":{"code":"unknown_lints","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/edition-lint-paths.rs","byte_start":503,"byte_end":537,"line_start":12,"line_end":12,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"#![deny(absolute_path_starting_with_module)]","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(unknown_lints)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unknown lint: `absolute_path_starting_with_module`\n  --> /checkout/src/test/ui/edition-lint-paths.rs:12:9\n   |\nLL | #![deny(absolute_path_starting_with_module)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unknown_lints)] on by default\n\n"}
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] thread '[ui] ui/edition-lint-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:46:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:07] 
[00:46:07] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:46:07] 
[00:46:07] 
[00:46:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:07] 
[00:46:07] 
[00:46:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:07] Build completed unsuccessfully in 0:02:25
[00:46:07] Build completed unsuccessfully in 0:02:25
[00:46:07] make: *** [check] Error 1
[00:46:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f4ff79c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
