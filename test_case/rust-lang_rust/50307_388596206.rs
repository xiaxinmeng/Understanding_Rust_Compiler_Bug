plain
[01:00:15] ............test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[01:00:45] ........................................................................................
[01:01:06] .....................................................................................ii.............
[01:02:04] .................................................i..................................................
[01:02:09] ..i.ii.........test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:03:02] ..........iiiiiii...................................................................................
[01:03:24] ....................................................................................................
[01:03:44] ....................................................................................................
[01:04:05] ...................................................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:48] 
[01:08:48] running 88 tests
[01:10:38] ..........................................FFF...........................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:14:11] failures:
[01:14:11] 
[01:14:11] ---- [run-pass] run-pass-fulldeps/plugin-args-1.rs stdout ----
[01:14:11]  
[01:14:11]  
[01:14:11] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile: 
[01:14:11] status: exit code: 101
[01:14:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1.stage2-x86_64-unknown-linux-gnu.aux"
[01:14:11] ------------------------------------------
[01:14:11] 
[01:14:11] ------------------------------------------
[01:14:11] stderr:
---
[01:14:11]    |     ^^^^^^^^^^^^^^
[01:14:11]    |
[01:14:11]    = note: #[warn(unused_imports)] on by default
[01:14:11] 
[01:14:11] error[E0063]: missing field `edition` in initializer of `syntax::ext::base::SyntaxExtension`
[01:14:11]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:51:9
[01:14:11] 51 |         NormalTT {
[01:14:11] 51 |         NormalTT {
[01:14:11]    |         ^^^^^^^^ missing `edition`
[01:14:11] error: aborting due to previous error
[01:14:11] 
[01:14:11] For more information about this error, try `rustc --explain E0063`.
[01:14:11] 
[01:14:11] 
[01:14:11] ------------------------------------------
[01:14:11] 
[01:14:11] thread '[run-pass] run-pass-fulldeps/plugin-args-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:14:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:14:11] 
[01:14:11] ---- [run-pass] run-pass-fulldeps/plugin-args-2.rs stdout ----
[01:14:11]  
[01:14:11] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile: 
[01:14:11] status: exit code: 101
[01:14:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-2.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-2.stage2-x86_64-unknown-linux-gnu.aux"
[01:14:11] ------------------------------------------
[01:14:11] 
[01:14:11] ------------------------------------------
[01:14:11] stderr:
---
[01:14:11]    |     ^^^^^^^^^^^^^^
[01:14:11]    |
[01:14:11]    = note: #[warn(unused_imports)] on by default
[01:14:11] 
[01:14:11] error[E0063]: missing field `edition` in initializer of `syntax::ext::base::SyntaxExtension`
[01:14:11]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:51:9
[01:14:11] 51 |         NormalTT {
[01:14:11] 51 |         NormalTT {
[01:14:11]    |         ^^^^^^^^ missing `edition`
[01:14:11] error: aborting due to previous error
[01:14:11] 
[01:14:11] For more information about this error, try `rustc --explain E0063`.
[01:14:11] 
[01:14:11] 
[01:14:11] ------------------------------------------
[01:14:11] 
[01:14:11] thread '[run-pass] run-pass-fulldeps/plugin-args-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:14:11] 
[01:14:11] ---- [run-pass] run-pass-fulldeps/plugin-args-3.rs stdout ----
[01:14:11]  
[01:14:11] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile: 
[01:14:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:14:11] status: exit code: 101
[01:14:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-3.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-3.stage2-x86_64-unknown-linux-gnu.aux"
[01:14:11] ------------------------------------------
[01:14:11] 
[01:14:11] ------------------------------------------
[01:14:11] stderr:
---
[01:14:11]    |     ^^^^^^^^^^^^^^
[01:14:11]    |
[01:14:11]    = note: #[warn(unused_imports)] on by default
[01:14:11] 
[01:14:11] error[E0063]: missing field `edition` in initializer of `syntax::ext::base::SyntaxExtension`
[01:14:11]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:51:9
[01:14:11] 51 |         NormalTT {
[01:14:11] 51 |         NormalTT {
[01:14:11]    |         ^^^^^^^^ missing `edition`
[01:14:11] error: aborting due to previous error
[01:14:11] 
[01:14:11] For more information about this error, try `rustc --explain E0063`.
[01:14:11] 
---
[01:14:11] test result: FAILED. 85 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:11] 
[01:14:11] 
[01:14:11] 
[01:14:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:11linux-gnu/release
41260 ./src/llvm/test/CodeGen/X86
40772 ./src/libcompiler_builtins
40268 ./src/libcompiler_builtins/compiler-rt
