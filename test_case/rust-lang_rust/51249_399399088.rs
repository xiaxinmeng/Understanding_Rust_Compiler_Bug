plain
[00:44:58] ....................................................................................................
[00:45:03] ........i...............................................................................i...........
[00:45:08] ....................................................................................................
[00:45:13] ....................................................................................................
[00:45:18] ...............................................................................................F....
[00:45:25] ..........
[00:45:25] failures:
[00:45:25] 
[00:45:25] ---- [ui (nll)] ui/suggestions/issue-51244.rs stdout ----
[00:45:25] ---- [ui (nll)] ui/suggestions/issue-51244.rs stdout ----
[00:45:25] diff of stderr:
[00:45:25] 
[00:45:25] - error[E0594]: cannot assign to immutable borrowed content `*my_ref`
[00:45:25] + error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference
[00:45:25] 2   --> $DIR/issue-51244.rs:13:5
[00:45:25] 3    |
[00:45:25] - LL |     let ref my_ref @ _ = 0;
[00:45:25] -    |         -------------- help: use a mutable reference instead: `ref mut my_ref @ _`
[00:45:25] 6 LL |     *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]
[00:45:25] -    |     ^^^^^^^^^^^ cannot borrow as mutable
[00:45:25] 8 
[00:45:25] 9 error: aborting due to previous error
[00:45:25] 10 
[00:45:25] 
[00:45:25] 
[00:45:25] 
[00:45:25] The actual stderr differed from the expected stderr.
[00:45:25] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/issue-51244.nll.stderr
[00:45:25] To update references, rerun the tests and pass the `--bless` flag
[00:45:25] To only update this specific test, also pass `--test-args suggestions/issue-51244.rs`
[00:45:25] error: 1 errors occurred comparing output.
[00:45:25] status: exit code: 101
[00:45:25] status: exit code: 101
[00:45:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/auxiliary" "-A" "unused"
[00:45:25] ------------------------------------------
[00:45:25] 
[00:45:25] ------------------------------------------
[00:45:25] stderr:
[00:45:25] stderr:
[00:45:25] ------------------------------------------
[00:45:25] {"message":"cannot assign to `*my_ref` which is behind a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":511,"byte_end":522,"line_start":13,"line_end":13,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]","highlight_start":5,"highlight_end":16}],"label":"cannot assign","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference\n  --> /checkout/src/test/ui/suggestions/issue-51244.rs:13:5\n   |\nLL |     *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]\n   |     ^^^^^^^^^^^ cannot assign\n\n"}
[00:45:25] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:25] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:45:25] ------------------------------------------
[00:45:25] 
[00:45:25] thread '[ui (nll)] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:25] 
[00:45:25] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:25] 
[00:45:25] 
[00:45:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:45:25] 
[00:45:25] 
[00:45:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:25] Build completed unsuccessfully in 0:03:16
[00:45:25] Build completed unsuccessfully in 0:03:16
[00:45:25] Makefile:58: recipe for target 'check' failed
[00:45:25] make: *** [check] Error 1
102864 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
102860 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
99484 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
91748 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
