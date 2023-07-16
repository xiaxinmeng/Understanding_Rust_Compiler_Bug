plain
[00:46:25] failures:
[00:46:25] 
[00:46:25] ---- [ui] ui/error-codes/E0088.rs stdout ----
[00:46:25] 
[00:46:25] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:46:25] status: exit code: 101
[00:46:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0088.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0088/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0088/auxiliary" "-A" "unused"
[00:46:25] ------------------------------------------
[00:46:25] 
[00:46:25] ------------------------------------------
[00:46:25] stderr:
[00:46:25] stderr:
[00:46:25] ------------------------------------------
[00:46:25] {"message":"too many lifetime parameters provided: expected at moe parameters\n\n"}
[00:46:25] {"message":"librustc_typeck/check/mod.rs:5036: GenericArg did not have matching GenericParamDef","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_typeck/check/mod.rs:5036: GenericArg did not have matching GenericParamDef\n\n"}
[00:46:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:25] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:25] {"message":"For more information about this error, try `rustc --explain E0088`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0088`.\n"}
[00:46:25] 
[00:46:25] note: the compiler unexpectedly panicked. this is a bug.
[00:46:25] 
[00:46:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:46:25] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:46:25] 
[00:46:25] 
[00:46:25] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:46:25] 
[00:46:25] ------------------------------------------
[00:46:25] 
[00:46:25] thread '[ui] ui/error-codes/E0088.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
---
[00:46:25] 
[00:46:25] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:46:25] 
[00:46:25] 
[00:46:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:25] 
[00:46:25] 
[00:46:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:25] Build completed unsu3609344 .
---
145372 ./obj/build/bootstrap/debug/incremental
133272 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133268 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130560 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130556 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f36i902gap-12ltfpu-3om13ebi8plfd
128708 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122328 ./obj/build/x86_64-un/compiler-rt/objects
34052 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
33908 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build
