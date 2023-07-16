plain
[01:00:31] running 1400 tests
[01:00:37] ..................................................................................i.................
[01:00:44] ................................i...................................................................
[01:00:49] ....................................................................................................
[01:00:53] ........F.F.........................................................................................
[01:01:02] ....................................................................................................
[01:01:09] ....................................................................................................
[01:01:16] ....................................................................................................
[01:01:16] ....................................................................................................
[01:01:23] .............................................................................F......................
[01:01:37] .............i......................................................................................
[01:01:44] ................................ii..................................................................
[01:01:53] ....................................................................................................
_BACKTRACE=1` for a backtrace.
_BACKTRACE=1` for a backtrace.
[01:02:01] 
[01:02:01] error: internal compiler error: unexpected panic
[01:02:01] 
[01:02:01] note: the compiler unexpectedly panicked. this is a bug.
[01:02:01] 
[01:02:01] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:01] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[01:02:01] 
[01:02:01] 
[01:02:01] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:02:01] 
[01:02:01] ------------------------------------------
[01:02:01] 
[01:02:01] thread '[ui] ui/error-codes/E0267.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:02:01] thread '[ui] ui/error-codes/E0267.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:02:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:01] 
[01:02:01] ---- [ui] ui/error-codes/E0268.rs stdout ----
[01:02:01]  diff of stderr:
[01:02:01] 
[01:02:01] - error[E0268]: `break` outside of loop
[01:02:01] -   --> $DIR/E0268.rs:12:5
[01:02:01] -    |
[01:02:01] - LL |     break; //~ ERROR E0268
[01:02:01] -    |     ^^^^^ cannot break outside of a loop
[01:02:01] - error: aborting due to previous error
[01:02:01] - 
[01:02:01] - For more information about this error, try `rustc --explain E0268`.
[01:02:01] - 
[01:02:01] - 
[01:02:01] 
[01:02:01] 
[01:02:01] The actual stderr differed from the expected stderr.
[01:02:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0268.stderr
[01:02:01] To update references, run this command from build directory:
[01:02:01] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'error-codes/E0268.rs'
[01:02:01] error: 1 errors occurred comparing output.
[01:02:01] status: exit code: 101
[01:02:01] status: exit code: 101
[01:02:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0268.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0268.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0268.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:01] ------------------------------------------
[01:02:01] 
[01:02:01] ------------------------------------------
[01:02:01] stderr:
[01:02:01] stderr:
[01:02:01] ------------------------------------------
[01:02:01] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:335:21
[01:02:01] 
[01:02:01] error: internal compiler error: unexpected panic
[01:02:01] 
[01:02:01] 
[01:02:01] note: the compiler unexpectedly panicked. this is a bug.
[01:02:01] 
[01:02:01] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:01] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[01:02:01] 
[01:02:01] 
[01:02:01] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:02:01] 
[01:02:01] ------------------------------------------
[01:02:01] 
[01:02:01] thread '[ui] ui/error-codes/E0268.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:02:01] thread '[ui] ui/error-codes/E0268.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:02:01] 
[01:02:01] ---- [ui] ui/loop-break-value-no-repeat.rs stdout ----
[01:02:01]  diff of stderr:
[01:02:01] 
[01:02:01] 2   --> $DIR/loop-break-value-no-repeat.rs:22:9
[01:02:01] 3    |
[01:02:01] 4 LL |         break 22 //~ ERROR `break` with value from a `for` loop
[01:02:01] -    |         ^^^^^^^^ can only break with a value inside `loop`
[01:02:01] +    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
[01:02:01] 6 help: instead, use `break` on its own without a value inside this `for` loop
[01:02:01] 7    |
[01:02:01] 8 LL |         break //~ ERROR `break` with value from a `for` loop
[01:02:01] 
[01:02:01] The actual stderr differed from the expected stderr.
[01:02:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loop-break-value-no-repeat.stderr
[01:02:01] To update references, run this command from build directory:
[01:02:01] To update references, run this command from build directory:
[01:02:01] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'loop-break-value-no-repeat.rs'
[01:02:01] 
[01:02:01] error: 1 errors occurred comparing output.
[01:02:01] status: exit code: 101
[01:02:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loop-break-value-no-repeat.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linuxyte_end":779,"line_start":22,"line_end":22,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        break 22 //~ ERROR `break` with value from a `for` loop","highlight_start":9,"highlight_end":17}],"label":"can only break with a value inside `loop` or breakable block","suggested_replacement":null,"expansion":null}],"children":[{"message":"instead, use `break` on its own without a value inside this `for` loop","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/loop-break-value-no-repeat.rs","byte_start":771,"byte_end":779,"line_start":22,"line_end":22,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        break 22 //~ ERROR `break` with value from a `for` loop","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":"break","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0571]: `break` with value from a `for` loop\n  --> /checkout/src/test/ui/loop-break-value-no-repeat.rs:22:9\n   |\nLL |         break 22 //~ ERROR `break` with value from a `for` loop\n   |         ^^^^^^^^ can only break with a value inside `loop` or breakable block\nhelp: instead, use `break` on its own without a value inside this `for` loop\n   |\nLL |         break //~ ERROR `break` with value from a `for` loop\n   |         ^^^^^\n\n"}
[01:02:01] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:02:01] {"message":"For more information about this error, try `rustc --explain E0571`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0571`.\n"}
[01:02:01] ------------------------------------------
[01:02:01] 
[01:02:01] thread '[ui] ui/loop-break-value-no-repeat.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:02:01] 
---
[01:02:01] 
[01:02:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:02:01] 
[01:02:01] 
[01:02:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python4018160 .
2743948 ./obj/build
1968892 ./obj/build/x86_64-unknown-linux-gnu
725932 ./src
584840 ./obj/build/bootstrap
---
149772 ./.git/modules
149768 ./.git/modules/src
149120 ./src/llvm-emscripten/test
124332 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124328 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f0yk5hp6qz-148nur-15rj8twogke02
