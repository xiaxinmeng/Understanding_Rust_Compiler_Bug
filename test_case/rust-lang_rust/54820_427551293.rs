plain
[00:49:59] ................................................................................................i... 2200/4557
[00:50:04] .................................................................................................... 2300/4557
[00:50:08] .................................................................................................... 2400/4557
[00:50:12] .................................................................................................... 2500/4557
[00:50:15] ........iiiiiiiii................................................................................... 2600/4557
[00:50:22] .................................................................................................... 2800/4557
[00:50:26] .................................................................................................... 2900/4557
[00:50:29] .............................i...................................................................... 3000/4557
[00:50:32] .........................................................................................i.i..ii.... 3100/4557
---
[00:51:19] 
[00:51:19] running 2872 tests
[00:51:29] .................................................................................................... 100/2872
[00:51:39] .............................................................................i...................... 200/2872
[00:51:47] ................................................................................F................... 300/2872
[00:52:04] .................................................................................................... 500/2872
[00:52:15] .................................................................................................... 600/2872
[00:52:29] .................................................................................................... 700/2872
[00:52:39] .................................................................................................... 800/2872
---
[00:56:49] normalized stderr:
[00:56:49] warning: unnecessary parentheses around pattern
[00:56:49]   --> $DIR/range-inclusive-pattern-precedence.rs:18:10
[00:56:49]    |
[00:56:49] LL |         &(18..=18) => {}
[00:56:49]    |
[00:56:49]    = note: #[warn(unused_parens)] on by default
[00:56:49] 
[00:56:49] warning: unnecessary parentheses around pattern
[00:56:49] warning: unnecessary parentheses around pattern
[00:56:49]   --> $DIR/range-inclusive-pattern-precedence.rs:22:10
[00:56:49]    |
[00:56:49] LL |         &(VALUE..=VALUE) => {}
[00:56:49] 
[00:56:49] warning: unnecessary parentheses around pattern
[00:56:49]   --> $DIR/range-inclusive-pattern-precedence.rs:26:13
[00:56:49]    |
[00:56:49]    |
[00:56:49] LL |         box (18..=18) => {}
[00:56:49] 
[00:56:49] warning: unnecessary parentheses around pattern
[00:56:49]   --> $DIR/range-inclusive-pattern-precedence.rs:30:13
[00:56:49]    |
[00:56:49]    |
[00:56:49] LL |         box (VALUE..=VALUE) => {}
[00:56:49] 
[00:56:49] 
[00:56:49] 
[00:56:49] 
[00:56:49] 
[00:56:49] The actual stderr differed from the expected stderr.
[00:56:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/range-inclusive-pattern-precedence/range-inclusive-pattern-precedence.stderr
[00:56:49] To update references, rerun the tests and pass the `--bless` flag
[00:56:49] To only update this specific test, also pass `--test-args binding/range-inclusive-pattern-precedence.rs`
[00:56:49] error: 1 errors occurred comparing output.
[00:56:49] status: exit code: 0
[00:56:49] status: exit code: 0
[00:56:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/range-inclusive-pattern-precedence/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x{}\n   |          ^^^^^^^^^ help: remove these parentheses\n   |\n   = note: #[warn(unused_parens)] on by default\n\n"}
[00:56:49] {"message":"unnecessary parentheses around pattern","code":{"code":"unused_parens","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":653,"byte_end":668,"line_start":22,"line_end":22,"column_start":10,"column_end":25,"is_primary":true,"text":[{"text":"        &(VALUE..=VALUE) => {}","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove these parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":653,"byte_end":668,"line_start":22,"line_end":22,"column_start":10,"column_end":25,"is_primary":true,"text":[{"text":"        &(VALUE..=VALUE) => {}","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":"VALUE ..=VALUE","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unnecessary parentheses around pattern\n  --> /checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs:22:10\n   |\nLL |         &(VALUE..=VALUE) => {}\n   |          ^^^^^^^^^^^^^^^ help: remove these parentheses\n\n"}
[00:56:49] {"message":"unnecessary parentheses around pattern","code":{"code":"unused_parens","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":751,"byte_end":760,"line_start":26,"line_end":26,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"        box (18..=18) => {}","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove these parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":751,"byte_end":760,"line_start":26,"line_end":26,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"        box (18..=18) => {}","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"18 ..=18","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unnecessary parentheses around pattern\n  --> /checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs:26:13\n   |\nLL |         box (18..=18) => {}\n   |             ^^^^^^^^^ help: remove these parentheses\n\n"}
[00:56:49] {"message":"unnecessary parentheses around pattern","code":{"code":"unused_parens","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":843,"byte_end":858,"line_start":30,"line_end":30,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"        box (VALUE..=VALUE) => {}","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove these parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":843,"byte_end":858,"line_start":30,"line_end":30,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"        box (VALUE..=VALUE) => {}","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":"VALUE ..=VALUE","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unnecessary parentheses around pattern\n  --> /checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs:30:13\n   |\nLL |         box (VALUE..=VALUE) => {}\n   |             ^^^^^^^^^^^^^^^ help: remove these parentheses\n\n"}
[00:56:49] ------------------------------------------
[00:56:49] 
[00:56:49] thread '[run-pass] run-pass/binding/range-inclusive-pattern-precedence.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:56:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:49] 
[00:56:49] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:56:49] 
[00:56:49] 
[00:56:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:49] 
[00:56:49] 
[00:56:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:49] Build completed unsuccessfully in 0:11:47
[00:56:49] Build completed unsuccessfully in 0:11:47
[00:56:49] Makefile:58: recipe for target 'check' failed
[00:56:49] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0758f604
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
