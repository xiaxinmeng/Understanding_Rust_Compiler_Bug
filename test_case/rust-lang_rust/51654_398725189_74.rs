\n\nSee [RFC 1522] for more details.\n\n[RFC 1522]: https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/compile-fail/impl-trait/where-allowed.rs","byte_start":7783,"byte_end":7792,"line_start":232,"line_end":232,"column_start":46,"column_end":55,"is_primary":true,"text":[{"text":"    let _in_return_in_local_variable = || -> impl Fn() { || {} };","highlight_start":46,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0562]: `impl Trait` not allowed outside of function and inherent method return types\n  --> /checkout/src/test/compile-fail/impl-trait/where-allowed.rs:232:46\n   |\nLL |     let _in_return_in_local_variable = || -> impl Fn() { || {} };\n   |                                              ^^^^^^^^^\n\n"}
[00:57:09] thread 'main' panicked at 'no entry found for key', libcore/option.rs:960:5
[00:57:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:09] {"message":"aborting due to 39 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 39 previous errors\n\n"}
[00:57:09] {"message":"Some errors occurred: E0562, E0666.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0562, E0666.\n"}
[00:57:09] {"message":"For more information about an error, try `rustc --explain E0562`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0562`.\n"}
[00:57:09] error: internal compiler error: unexpected panic
[00:57:09] 
[00:57:09] 
[00:57:09] note: the compiler unexpectedly panicked. this is a bug.
[00:57:09] 
[00:57:09] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:09] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:57:09] 
[00:57:09] 
[00:57:09] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:57:09] 
[00:57:09] ------------------------------------------
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/impl-trait/where-allowed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:57:09] thread '[compile-fail] compile-fail/impl-trait/where-allowed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:57:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:09] 
[00:57:09] ---- [compile-fail] compile-fail/private-type-in-interface.rs stdout ----
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/private-type-in-interface.rs:37: unexpected error: '37:16: 37:29: type `m::Priv` is private'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/private-type-in-interface.rs:38: unexpected error: '38:20: 38:35: type `ext::Priv` is private'
[00:57:09] 
[00:57:09] error: 2 unexpected errors found, 0 expected errors not found
[00:57:09] status: exit code: 101
[00:57:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/private-type-in-interface.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/private-type-in-interface/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/private-type-in-interface/auxiliary" "-A" "unused"
[00:57:09] unexpected errors (from JSON output): [
[00:57:09]     Error {
[00:57:09]         line_num: 37,
[00:57:09]         kind: Some(
[00:57:09]         ),
[00:57:09]         ),
[00:57:09]         msg: "37:16: 37:29: type `m::Priv` is private"
[00:57:09]     Error {
[00:57:09]         line_num: 38,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "38:20: 38:35: type `ext::Priv` is private"
[00:57:09] ]
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/private-type-in-interface.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:57:09] 
---
[00:57:09] test result: FAILED. 2409 passed; 2 failed; 15 ignored; 0 measured; 0 filtered out
[00:57:09] 
[00:57:09] 
[00:57:09] 
[00:57:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:09] 
[00:57:09] 
[00:57:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:09] Build completed unsuccessfully in 0:14:35
[00:57:09] Build completed unsuccessfully in 0:14:35
[00:57:09] Makefile:58: recipe for target 'check' failed
[00:57:09] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24836a60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
