plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:38] 
[00:59:38] running 88 tests
[01:01:08] ................................F.F.FF......FF....................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:03:21] failures:
[01:03:21] 
[01:03:21] ---- [run-pass] run-pass-fulldeps/macro-crate-does-hygiene-work.rs stdout ----
[01:03:21]  
[01:03:21]  
[01:03:21] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:03:21] status: exit code: 101
[01:03:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-does-hygiene-work.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-does-hygiene-work.stage2-x86_64-unknown-linux-gnu.aux"
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] stderr:
[01:03:21] stderr:
[01:03:21] ------------------------------------------
[01:03:21] warning: unused macro definition
[01:03:21]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:03:21]    |
[01:03:21] 33 | macro_rules! unexported_macro { () => (3) }
[01:03:21]    |
[01:03:21]    = note: #[warn(unused_macros)] on by default
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:132:28
[01:03:21]     |
[01:03:21] 132 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:138:28
[01:03:21]     |
[01:03:21] 138 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:144:28
[01:03:21]     |
[01:03:21] 144 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
---
[01:03:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:21] 
[01:03:21] ---- [run-pass] run-pass-fulldeps/macro-crate-multi-decorator-literals.rs stdout ----
[01:03:21]  
[01:03:21] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:03:21] status: exit code: 101
[01:03:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu.aux"
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] stderr:
[01:03:21] stderr:
[01:03:21] ------------------------------------------
[01:03:21] warning: unused macro definition
[01:03:21]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:03:21]    |
[01:03:21] 33 | macro_rules! unexported_macro { () => (3) }
[01:03:21]    |
[01:03:21]    = note: #[warn(unused_macros)] on by default
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:132:28
[01:03:21]     |
[01:03:21] 132 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:138:28
[01:03:21]     |
[01:03:21] 138 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:144:28
[01:03:21]     |
[01:03:21] 144 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error: aborting due to 3 previous errors
[01:03:21] 
[01:03:21] For more information about this error, try `rustc --explain E0308`.
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] thread '[run-pass] run-pass-fulldeps/macro-crate-multi-decorator-literals.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[01:03:21] ---- [run-pass] run-pass-fulldeps/macro-crate-multi-decorator.rs stdout ----
[01:03:21]  
[01:03:21]  
[01:03:21] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:03:21] status: exit code: 101
[01:03:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator.stage2-x86_64-unknown-linux-gnu.aux"
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] stderr:
[01:03:21] stderr:
[01:03:21] ------------------------------------------
[01:03:21] warning: unused macro definition
[01:03:21]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:03:21]    |
[01:03:21] 33 | macro_rules! unexported_macro { () => (3) }
[01:03:21]    |
[01:03:21]    = note: #[warn(unused_macros)] on by default
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:132:28
[01:03:21]     |
[01:03:21] 132 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:138:28
[01:03:21]     |
[01:03:21] 138 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:144:28
[01:03:21]     |
[01:03:21] 144 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error: aborting due to 3 previous errors
[01:03:21] 
[01:03:21] For more information about this error, try `rustc --explain E0308`.
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] thread '[run-pass] run-pass-fulldeps/macro-crate-multi-decorator.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[01:03:21] ---- [run-pass] run-pass-fulldeps/macro-crate.rs stdout ----
[01:03:21]  
[01:03:21]  
[01:03:21] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:03:21] status: exit code: 101
[01:03:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate.stage2-x86_64-unknown-linux-gnu.aux"
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] stderr:
[01:03:21] stderr:
[01:03:21] ------------------------------------------
[01:03:21] warning: unused macro definition
[01:03:21]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:03:21]    |
[01:03:21] 33 | macro_rules! unexported_macro { () => (3) }
[01:03:21]    |
[01:03:21]    = note: #[warn(unused_macros)] on by default
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:132:28
[01:03:21]     |
[01:03:21] 132 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:138:28
[01:03:21]     |
[01:03:21] 138 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:144:28
[01:03:21]     |
[01:03:21] 144 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
---
[01:03:21] thread '[run-pass] run-pass-fulldeps/macro-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[01:03:21] 
[01:03:21] ---- [run-pass] run-pass-fulldeps/plugin-lib-ok-in-plugin.rs stdout ----
[01:03:21]  
[01:03:21] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:03:21] status: exit code: 101
[01:03:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-lib-ok-in-plugin.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-lib-ok-in-plugin.stage2-x86_64-unknown-linux-gnu.aux"
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] stderr:
[01:03:21] stderr:
[01:03:21] ------------------------------------------
[01:03:21] warning: unused macro definition
[01:03:21]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:03:21]    |
[01:03:21] 33 | macro_rules! unexported_macro { () => (3) }
[01:03:21]    |
[01:03:21]    = note: #[warn(unused_macros)] on by default
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:132:28
[01:03:21]     |
[01:03:21] 132 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:138:28
[01:03:21]     |
[01:03:21] 138 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:144:28
[01:03:21]     |
[01:03:21] 144 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error: aborting due to 3 previous errors
[01:03:21] 
[01:03:21] For more information about this error, try `rustc --explain E0308`.
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] thread '[run-pass] run-pass-fulldeps/plugin-lib-ok-in-plugin.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[01:03:21] ---- [run-pass] run-pass-fulldeps/plugin-plus-extern-crate.rs stdout ----
[01:03:21]  
[01:03:21]  
[01:03:21] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:03:21] status: exit code: 101
[01:03:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-plus-extern-crate.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-plus-extern-crate.stage2-x86_64-unknown-linux-gnu.aux"
[01:03:21] ------------------------------------------
[01:03:21] 
[01:03:21] ------------------------------------------
[01:03:21] stderr:
[01:03:21] stderr:
[01:03:21] ------------------------------------------
[01:03:21] warning: unused macro definition
[01:03:21]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:03:21]    |
[01:03:21] 33 | macro_rules! unexported_macro { () => (3) }
[01:03:21]    |
[01:03:21]    = note: #[warn(unused_macros)] on by default
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:132:28
[01:03:21]     |
[01:03:21] 132 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:138:28
[01:03:21]     |
[01:03:21] 138 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
[01:03:21] 
[01:03:21] error[E0308]: mismatched types
[01:03:21]    --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:144:28
[01:03:21]     |
[01:03:21] 144 |             new_it.ident = copy_name;
[01:03:21]     |
[01:03:21]     = note: expected type `syntax::ast::Ident`
[01:03:21]                found type `syntax::ast::Path`
[01:03:21] 
---
[01:03:21] test result: FAILED. 82 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:21] 
[01:03:21] 
[01:03:21] 
[01:03:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:21] 
[01:03:21] 
[01:03:21] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:03:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:21] Build completed unsuccessfully in 0:22:27
[01:03:21] make: *** [check] Error 1
[01:03:21] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:032e7c58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
