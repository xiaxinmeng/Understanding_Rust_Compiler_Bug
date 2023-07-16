plain
2019-07-19T14:12:09.3712988Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T14:12:09.3713042Z 
2019-07-19T14:12:09.3713282Z   git checkout -b <new-branch-name>
2019-07-19T14:12:09.3713327Z 
2019-07-19T14:12:09.3713632Z HEAD is now at 1091349b3 Auto merge of #62797 - petrochenkov:run-pass-hole, r=Mark-Simulacrum
2019-07-19T14:12:09.3865743Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T14:12:09.3868507Z ==============================================================================
2019-07-19T14:12:09.3868609Z Task         : Bash
2019-07-19T14:12:09.3868672Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T15:47:51.8482480Z ---- [run-pass] run-pass/generator/size-moved-locals.rs stdout ----
2019-07-19T15:47:51.8482558Z 
2019-07-19T15:47:51.8482620Z error: test run failed!
2019-07-19T15:47:51.8482703Z status: exit code: 101
2019-07-19T15:47:51.8483190Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/size-moved-locals/a.wasm"
2019-07-19T15:47:51.8483809Z ------------------------------------------
2019-07-19T15:47:51.8483875Z 
2019-07-19T15:47:51.8484103Z ------------------------------------------
2019-07-19T15:47:51.8484188Z stderr:
2019-07-19T15:47:51.8484188Z stderr:
2019-07-19T15:47:51.8484415Z ------------------------------------------
2019-07-19T15:47:51.8484502Z RuntimeError: unreachable
2019-07-19T15:47:51.8484753Z     at __rust_start_panic (wasm-function[65]:1)
2019-07-19T15:47:51.8485005Z     at rust_panic (wasm-function[60]:39)
2019-07-19T15:47:51.8485317Z     at _ZN3std9panicking20rust_panic_with_hook17hb64f1aeb1f625a2aE (wasm-function[55]:346)
2019-07-19T15:47:51.8485649Z     at _ZN3std9panicking18continue_panic_fmt17h8bffa35d05d676f9E (wasm-function[54]:151)
2019-07-19T15:47:51.8488784Z     at _ZN3std9panicking15begin_panic_fmt17h31b07c0c65ebceb5E (wasm-function[27]:108)
2019-07-19T15:47:51.8489246Z     at _ZN17size_moved_locals4main17h02cd372238a9883dE (wasm-function[0]:144)
2019-07-19T15:47:51.8489585Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h72d5442d7ee73094E (wasm-function[3]:25)
2019-07-19T15:47:51.8489962Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he1adeffeb91504b7E (wasm-function[43]:8)
2019-07-19T15:47:51.8490266Z     at _ZN3std9panicking3try7do_call17h0b8ce620099b0dacE (wasm-function[52]:20)
2019-07-19T15:47:51.8490551Z     at __rust_maybe_catch_panic (wasm-function[64]:5)
2019-07-19T15:47:51.8490861Z     at _ZN3std2rt19lang_start_internal17h07703b904ab9ad19E (wasm-function[61]:264)
2019-07-19T15:47:51.8491106Z     at main (wasm-function[1]:46)
2019-07-19T15:47:51.8491567Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:126:20)
2019-07-19T15:47:51.8491664Z     at Module._compile (module.js:641:30)
2019-07-19T15:47:51.8491759Z     at Object.Module._extensions..js (module.js:652:10)
2019-07-19T15:47:51.8491833Z     at Module.load (module.js:560:32)
2019-07-19T15:47:51.8491920Z     at tryModuleLoad (module.js:503:12)
2019-07-19T15:47:51.8491992Z     at Function.Module._load (module.js:495:3)
2019-07-19T15:47:51.8492091Z     at Function.Module.runMain (module.js:682:10)
2019-07-19T15:47:51.8492163Z     at startup (bootstrap_node.js:191:16)
2019-07-19T15:47:51.8492501Z ------------------------------------------
2019-07-19T15:47:51.8492547Z 
2019-07-19T15:47:51.8492599Z 
2019-07-19T15:47:51.8492633Z 
---
2019-07-19T15:47:51.8493892Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-19T15:47:51.8494004Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-19T15:47:51.8494332Z 
2019-07-19T15:47:51.8496304Z 
2019-07-19T15:47:51.8500791Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-pass" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-19T15:47:51.8505340Z 
2019-07-19T15:47:51.8505379Z 
2019-07-19T15:47:51.8506453Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-07-19T15:47:51.8506613Z Build completed unsuccessfully in 1:25:41
2019-07-19T15:47:51.8506613Z Build completed unsuccessfully in 1:25:41
2019-07-19T15:47:52.2675323Z ##[error]Bash exited with code '1'.
2019-07-19T15:47:52.2714696Z ##[section]Starting: Upload CPU usage statistics
2019-07-19T15:47:52.2726812Z ==============================================================================
2019-07-19T15:47:52.2726934Z Task         : Bash
2019-07-19T15:47:52.2727020Z Description  : Run a Bash script on macOS, Linux, or Windows
