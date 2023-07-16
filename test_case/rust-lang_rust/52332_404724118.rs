plain
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:42] 
[00:51:42] running 21 tests
[00:51:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:58] ......F..............
[00:51:58] 
[00:51:58] ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
[00:51:58] diff of stderr:
[00:51:58] 
[00:51:58] 
[00:51:58] - warning: function is never used: `lintme`
[00:51:58] + warning: function is never called: `lintme`
[00:51:58] 3    |
[00:51:58] 3    |
[00:51:58] 4 LL | fn lintme() { }
[00:51:58] 
[00:51:58] The actual stderr differed from the expected stderr.
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/lint-plugin-cmdline-allow.stderr
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/lint-plugin-cmdline-allow.stderr
[00:51:58] To update references, rerun the tests and pass the `--bless` flag
[00:51:58] To only update this specific test, also pass `--test-args lint-plugin-cmdline-allow.rs`
[00:51:58] error: 1 errors occurred comparing output.
[00:51:58] status: exit code: 0
[00:51:58] status: exit code: 0
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-A" "unused"
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] ------------------------------------------
[00:51:58] stderr:
[00:51:58] stderr:
[00:51:58] ------------------------------------------
[00:51:58] {"message":"function is never called: `lintme`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs","byte_start":628,"byte_end":639,"line_start":20,"line_end":20,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"fn lintme() { }","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs","byte_start":589,"byte_end":595,"line_start":17,"line_end":17,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"#![warn(unused)]","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[warn(dead_code)] implied by #[warn(unused)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function is never called: `lintme`\n  --> /checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs:20:1\n   |\nLL | fn lintme() { }\n   | ^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs:17:9\n   |\nLL | #![warn(unused)]\n   |         ^^^^^^\n   = note: #[warn(dead_code)] implied by #[warn(unused)]\n\n"}
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] thread '[ui] ui-fulldeps/lint-plugin-cmdline-allow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:51:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:58] test result: FAILED. 20 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:51:58] 
[00:51:58] 
[00:51:58] 
[00:51:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:58] 
[00:51:58] 
[00:51:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:58] Build completed unsuccessfully in 0:10:07
[00:51:58] Build completed unsuccessfully in 0:10:07
[00:51:58] make: *** [check] Error 1
[00:51:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:28c9e81c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
