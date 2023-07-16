plain
2020-01-06T17:27:55.3630101Z ---- [ui] ui/rfc-2091-track-caller/std-panic-locations.rs stdout ----
2020-01-06T17:27:55.3630215Z 
2020-01-06T17:27:55.3630362Z error: test run failed!
2020-01-06T17:27:55.3630495Z status: exit code: 101
2020-01-06T17:27:55.3631363Z command: "/node-v9.2.0-linux-x64/bin/node" "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations/a.wasm"
2020-01-06T17:27:55.3632148Z ------------------------------------------
2020-01-06T17:27:55.3632241Z 
2020-01-06T17:27:55.3632682Z ------------------------------------------
2020-01-06T17:27:55.3632816Z stderr:
2020-01-06T17:27:55.3632816Z stderr:
2020-01-06T17:27:55.3633250Z ------------------------------------------
2020-01-06T17:27:55.3633410Z RuntimeError: unreachable
2020-01-06T17:27:55.3633866Z     at __rust_start_panic (wasm-function[107]:1)
2020-01-06T17:27:55.3634332Z     at rust_panic (wasm-function[102]:39)
2020-01-06T17:27:55.3634911Z     at _ZN3std9panicking20rust_panic_with_hook17h9c4965b6e447926bE (wasm-function[97]:275)
2020-01-06T17:27:55.3635421Z     at rust_begin_unwind (wasm-function[96]:90)
2020-01-06T17:27:55.3635969Z     at _ZN4core9panicking9panic_fmt17h8bd2f15181c43e19E (wasm-function[127]:54)
2020-01-06T17:27:55.3636544Z     at _ZN4core9panicking5panic17hbd4797345db79641E (wasm-function[122]:62)
2020-01-06T17:27:55.3637192Z     at _ZN3std9panicking3try7do_call17hee765e82e61dc928E.llvm.4709490267703860165 (wasm-function[6]:27)
2020-01-06T17:27:55.3637724Z     at __rust_maybe_catch_panic (wasm-function[106]:5)
2020-01-06T17:27:55.3638289Z     at _ZN19std_panic_locations4main17h42204a682dbb00c5E (wasm-function[21]:94)
2020-01-06T17:27:55.3638905Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h948cd167741f81e6E (wasm-function[0]:25)
2020-01-06T17:27:55.3639564Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hb4a3364a6054932fE (wasm-function[84]:8)
2020-01-06T17:27:55.3640140Z     at _ZN3std9panicking3try7do_call17h12e19305fca28510E (wasm-function[95]:20)
2020-01-06T17:27:55.3640988Z     at __rust_maybe_catch_panic (wasm-function[106]:5)
2020-01-06T17:27:55.3641581Z     at _ZN3std2rt19lang_start_internal17h95dd2a1d1a2552e5E (wasm-function[103]:250)
2020-01-06T17:27:55.3642057Z     at main (wasm-function[22]:46)
2020-01-06T17:27:55.3642591Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2020-01-06T17:27:55.3642752Z     at Module._compile (module.js:641:30)
2020-01-06T17:27:55.3642924Z     at Object.Module._extensions..js (module.js:652:10)
2020-01-06T17:27:55.3643066Z     at Module.load (module.js:560:32)
2020-01-06T17:27:55.3643219Z     at tryModuleLoad (module.js:503:12)
2020-01-06T17:27:55.3643740Z ------------------------------------------
2020-01-06T17:27:55.3643831Z 
2020-01-06T17:27:55.3643899Z 
2020-01-06T17:27:55.3643966Z 
---
2020-01-06T17:27:55.3646257Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-06T17:27:55.3646468Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-06T17:27:55.3646729Z 
2020-01-06T17:27:55.3646795Z 
2020-01-06T17:27:55.3650504Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-06T17:27:55.3651723Z 
2020-01-06T17:27:55.3651813Z 
2020-01-06T17:27:55.3652611Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-01-06T17:27:55.3652864Z Build completed unsuccessfully in 1:10:38
2020-01-06T17:27:55.3652864Z Build completed unsuccessfully in 1:10:38
2020-01-06T17:27:55.3653018Z == clock drift check ==
2020-01-06T17:27:55.3653168Z   local time: Mon Jan  6 17:27:55 UTC 2020
2020-01-06T17:27:55.5989130Z   network time: Mon, 06 Jan 2020 17:27:55 GMT
2020-01-06T17:27:55.5996695Z == end clock drift check ==
2020-01-06T17:27:56.9467012Z 
2020-01-06T17:27:56.9557842Z ##[error]Bash exited with code '1'.
2020-01-06T17:27:56.9602994Z ##[section]Starting: Checkout
2020-01-06T17:27:56.9605789Z ==============================================================================
2020-01-06T17:27:56.9605887Z Task         : Get sources
2020-01-06T17:27:56.9605992Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
