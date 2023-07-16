plain
2019-10-11T10:22:14.3520631Z ---- [ui] ui/consts/const-eval/write-to-uninhabited-enum-variant.rs stdout ----
2019-10-11T10:22:14.3520932Z 
2019-10-11T10:22:14.3521200Z error: test run failed!
2019-10-11T10:22:14.3521448Z status: exit code: 101
2019-10-11T10:22:14.3522315Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/a.wasm"
2019-10-11T10:22:14.3523260Z ------------------------------------------
2019-10-11T10:22:14.3523538Z 
2019-10-11T10:22:14.3523982Z ------------------------------------------
2019-10-11T10:22:14.3524310Z stderr:
2019-10-11T10:22:14.3524310Z stderr:
2019-10-11T10:22:14.3524762Z ------------------------------------------
2019-10-11T10:22:14.3525046Z RuntimeError: unreachable
2019-10-11T10:22:14.3526327Z     at _ZN3std7process4exit17h328f88c59e36dc5cE (wasm-function[41]:7)
2019-10-11T10:22:14.3527044Z     at _ZN33write_to_uninhabited_enum_variant3bar17h017db0f6c0883816E (wasm-function[3]:3)
2019-10-11T10:22:14.3527683Z     at _ZN33write_to_uninhabited_enum_variant4main17hced8f1a3a6cc2e0bE (wasm-function[4]:1)
2019-10-11T10:22:14.3528351Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hdb39e8fceb70f453E (wasm-function[0]:25)
2019-10-11T10:22:14.3529180Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h1ed57b0f73acd440E (wasm-function[45]:8)
2019-10-11T10:22:14.3529773Z     at _ZN3std9panicking3try7do_call17h501f0d80d0bbf033E (wasm-function[54]:20)
2019-10-11T10:22:14.3530314Z     at __rust_maybe_catch_panic (wasm-function[66]:5)
2019-10-11T10:22:14.3530883Z     at _ZN3std2rt19lang_start_internal17h3561a1458b6fa0ccE (wasm-function[63]:250)
2019-10-11T10:22:14.3531409Z     at main (wasm-function[5]:46)
2019-10-11T10:22:14.3532141Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-11T10:22:14.3533290Z     at Module._compile (module.js:641:30)
2019-10-11T10:22:14.3533545Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-11T10:22:14.3533737Z     at Module.load (module.js:560:32)
2019-10-11T10:22:14.3534086Z     at tryModuleLoad (module.js:503:12)
2019-10-11T10:22:14.3534426Z     at Function.Module._load (module.js:495:3)
2019-10-11T10:22:14.3534596Z     at Function.Module.runMain (module.js:682:10)
2019-10-11T10:22:14.3534750Z     at startup (bootstrap_node.js:191:16)
2019-10-11T10:22:14.3534917Z     at bootstrap_node.js:613:3
2019-10-11T10:22:14.3535983Z ------------------------------------------
2019-10-11T10:22:14.3536223Z 
2019-10-11T10:22:14.3536380Z 
2019-10-11T10:22:14.3536508Z 
---
2019-10-11T10:22:14.3544238Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-11T10:22:14.3544628Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-11T10:22:14.3557104Z 
2019-10-11T10:22:14.3557709Z 
2019-10-11T10:22:14.3561569Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-11T10:22:14.3562545Z 
2019-10-11T10:22:14.3562584Z 
2019-10-11T10:22:14.3571659Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-10-11T10:22:14.3598013Z Build completed unsuccessfully in 1:30:54
2019-10-11T10:22:14.3598013Z Build completed unsuccessfully in 1:30:54
2019-10-11T10:22:14.3630889Z == clock drift check ==
2019-10-11T10:22:14.3674883Z   local time: Fri Oct 11 10:22:14 UTC 2019
2019-10-11T10:22:14.5535192Z   network time: Fri, 11 Oct 2019 10:22:14 GMT
2019-10-11T10:22:14.5536746Z == end clock drift check ==
2019-10-11T10:22:15.7551504Z ##[error]Bash exited with code '1'.
2019-10-11T10:22:15.7611718Z ##[section]Starting: Upload CPU usage statistics
2019-10-11T10:22:15.7652403Z ==============================================================================
2019-10-11T10:22:15.7652535Z Task         : Bash
2019-10-11T10:22:15.7652632Z Description  : Run a Bash script on macOS, Linux, or Windows
