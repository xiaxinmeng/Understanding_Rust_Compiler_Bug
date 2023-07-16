plain
2019-11-26T20:37:20.6108571Z failures:
2019-11-26T20:37:20.6145265Z 
2019-11-26T20:37:20.6146117Z ---- [ui] ui/issues/issue-65419/issue-65419-async-fn-resume-after-completion.rs stdout ----
2019-11-26T20:37:20.6146243Z 
2019-11-26T20:37:20.6146644Z error: error pattern ' thread 'main' panicked at '`async fn` resumed after completion'' not found!
2019-11-26T20:37:20.6146768Z status: exit code: 101
2019-11-26T20:37:20.6147272Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65419/issue-65419-async-fn-resume-after-completion/a.wasm"
2019-11-26T20:37:20.6147858Z ------------------------------------------
2019-11-26T20:37:20.6147912Z 
2019-11-26T20:37:20.6148172Z ------------------------------------------
2019-11-26T20:37:20.6148246Z stderr:
2019-11-26T20:37:20.6148246Z stderr:
2019-11-26T20:37:20.6148499Z ------------------------------------------
2019-11-26T20:37:20.6148593Z RuntimeError: unreachable
2019-11-26T20:37:20.6148851Z     at __rust_start_panic (wasm-function[74]:1)
2019-11-26T20:37:20.6149119Z     at rust_panic (wasm-function[69]:39)
2019-11-26T20:37:20.6149453Z     at _ZN3std9panicking20rust_panic_with_hook17hac22899b3ad8c003E (wasm-function[64]:327)
2019-11-26T20:37:20.6149843Z     at _ZN3std9panicking18continue_panic_fmt17h2ed3c020db1eed81E (wasm-function[63]:151)
2019-11-26T20:37:20.6150143Z     at rust_begin_unwind (wasm-function[62]:3)
2019-11-26T20:37:20.6150486Z     at _ZN4core9panicking9panic_fmt17h2bb1cac5224c3c71E (wasm-function[92]:54)
2019-11-26T20:37:20.6150838Z     at _ZN4core9panicking5panic17hb0a26625597c3cedE (wasm-function[90]:62)
2019-11-26T20:37:20.6151235Z     at _ZN44issue_65419_async_fn_resume_after_completion8executor8block_on17hf16744be17ddedb5E (wasm-function[12]:138)
2019-11-26T20:37:20.6151634Z     at _ZN44issue_65419_async_fn_resume_after_completion4main17h3b2d8c640557c458E (wasm-function[0]:48)
2019-11-26T20:37:20.6152014Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3304a47c52e9820eE (wasm-function[2]:25)
2019-11-26T20:37:20.6152425Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hb0517212f4a7a6e1E (wasm-function[52]:8)
2019-11-26T20:37:20.6152789Z     at _ZN3std9panicking3try7do_call17h5f71630b133d918eE (wasm-function[61]:20)
2019-11-26T20:37:20.6153093Z     at __rust_maybe_catch_panic (wasm-function[73]:5)
2019-11-26T20:37:20.6153441Z     at _ZN3std2rt19lang_start_internal17h83f04431ca309805E (wasm-function[70]:250)
2019-11-26T20:37:20.6153713Z     at main (wasm-function[1]:46)
2019-11-26T20:37:20.6154036Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-26T20:37:20.6154134Z     at Module._compile (module.js:641:30)
2019-11-26T20:37:20.6154238Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-26T20:37:20.6154336Z     at Module.load (module.js:560:32)
2019-11-26T20:37:20.6154410Z     at tryModuleLoad (module.js:503:12)
2019-11-26T20:37:20.6154740Z ------------------------------------------
2019-11-26T20:37:20.6154793Z 
2019-11-26T20:37:20.6154849Z 
2019-11-26T20:37:20.6155174Z ---- [ui] ui/issues/issue-65419/issue-65419-async-fn-resume-after-panic.rs stdout ----
2019-11-26T20:37:20.6155174Z ---- [ui] ui/issues/issue-65419/issue-65419-async-fn-resume-after-panic.rs stdout ----
2019-11-26T20:37:20.6155241Z 
2019-11-26T20:37:20.6155580Z error: error pattern ' thread 'main' panicked at '`async fn` resumed after panicking'' not found!
2019-11-26T20:37:20.6155697Z status: exit code: 101
2019-11-26T20:37:20.6156167Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65419/issue-65419-async-fn-resume-after-panic/a.wasm"
2019-11-26T20:37:20.6156584Z ------------------------------------------
2019-11-26T20:37:20.6156656Z 
2019-11-26T20:37:20.6156911Z ------------------------------------------
2019-11-26T20:37:20.6157004Z stderr:
2019-11-26T20:37:20.6157004Z stderr:
2019-11-26T20:37:20.6157259Z ------------------------------------------
2019-11-26T20:37:20.6157355Z RuntimeError: unreachable
2019-11-26T20:37:20.6157629Z     at __rust_start_panic (wasm-function[83]:1)
2019-11-26T20:37:20.6157917Z     at rust_panic (wasm-function[78]:39)
2019-11-26T20:37:20.6158373Z     at _ZN3std9panicking20rust_panic_with_hook17hac22899b3ad8c003E (wasm-function[73]:327)
2019-11-26T20:37:20.6158765Z     at _ZN3std9panicking11begin_panic17hd1d73f4b2ad7b927E (wasm-function[0]:49)
2019-11-26T20:37:20.6161933Z     at _ZN39issue_65419_async_fn_resume_after_panic3foo28_$u7b$$u7b$closure$u7d$$u7d$17h42442869546dcd9cE.llvm.18431008703977981477 (wasm-function[10]:33)
2019-11-26T20:37:20.6162405Z     at _ZN80_$LT$std..future..GenFuture$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$4poll17hc3eb648ba76fb974E (wasm-function[9]:12)
2019-11-26T20:37:20.6163267Z     at _ZN3std9panicking3try7do_call17he6aa5a54cef84856E.llvm.9563853026639181194 (wasm-function[1]:76)
2019-11-26T20:37:20.6163604Z     at __rust_maybe_catch_panic (wasm-function[82]:5)
2019-11-26T20:37:20.6163966Z     at _ZN39issue_65419_async_fn_resume_after_panic4main17h6022587208e370edE (wasm-function[13]:104)
2019-11-26T20:37:20.6164368Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h20e354da0afff9a4E (wasm-function[6]:25)
2019-11-26T20:37:20.6164750Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hb0517212f4a7a6e1E (wasm-function[60]:8)
2019-11-26T20:37:20.6165117Z     at _ZN3std9panicking3try7do_call17h5f71630b133d918eE (wasm-function[70]:20)
2019-11-26T20:37:20.6165430Z     at __rust_maybe_catch_panic (wasm-function[82]:5)
2019-11-26T20:37:20.6165763Z     at _ZN3std2rt19lang_start_internal17h83f04431ca309805E (wasm-function[79]:250)
2019-11-26T20:37:20.6166059Z     at main (wasm-function[14]:46)
2019-11-26T20:37:20.6166373Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-26T20:37:20.6166483Z     at Module._compile (module.js:641:30)
2019-11-26T20:37:20.6166568Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-26T20:37:20.6166666Z     at Module.load (module.js:560:32)
2019-11-26T20:37:20.6166756Z     at tryModuleLoad (module.js:503:12)
2019-11-26T20:37:20.6167066Z ------------------------------------------
2019-11-26T20:37:20.6167148Z 
2019-11-26T20:37:20.6167185Z 
2019-11-26T20:37:20.6167526Z ---- [ui] ui/issues/issue-65419/issue-65419-generator-resume-after-completion.rs stdout ----
2019-11-26T20:37:20.6167526Z ---- [ui] ui/issues/issue-65419/issue-65419-generator-resume-after-completion.rs stdout ----
2019-11-26T20:37:20.6167596Z 
2019-11-26T20:37:20.6167880Z error: error pattern 'generator resumed after completion' not found!
2019-11-26T20:37:20.6167989Z status: exit code: 101
2019-11-26T20:37:20.6168757Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65419/issue-65419-generator-resume-after-completion/a.wasm"
2019-11-26T20:37:20.6169195Z ------------------------------------------
2019-11-26T20:37:20.6169249Z 
2019-11-26T20:37:20.6169519Z ------------------------------------------
2019-11-26T20:37:20.6169594Z stderr:
2019-11-26T20:37:20.6169594Z stderr:
2019-11-26T20:37:20.6169868Z ------------------------------------------
2019-11-26T20:37:20.6169946Z RuntimeError: unreachable
2019-11-26T20:37:20.6170249Z     at __rust_start_panic (wasm-function[64]:1)
2019-11-26T20:37:20.6170516Z     at rust_panic (wasm-function[59]:39)
2019-11-26T20:37:20.6170875Z     at _ZN3std9panicking20rust_panic_with_hook17hac22899b3ad8c003E (wasm-function[54]:327)
2019-11-26T20:37:20.6171239Z     at _ZN3std9panicking18continue_panic_fmt17h2ed3c020db1eed81E (wasm-function[53]:151)
2019-11-26T20:37:20.6171534Z     at rust_begin_unwind (wasm-function[52]:3)
2019-11-26T20:37:20.6171870Z     at _ZN4core9panicking9panic_fmt17h2bb1cac5224c3c71E (wasm-function[82]:54)
2019-11-26T20:37:20.6172214Z     at _ZN4core9panicking5panic17hb0a26625597c3cedE (wasm-function[80]:62)
2019-11-26T20:37:20.6172598Z     at _ZN45issue_65419_generator_resume_after_completion4main17h2cd563fc458fa343E (wasm-function[3]:15)
2019-11-26T20:37:20.6172987Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h31e34ece10dc7b36E (wasm-function[0]:25)
2019-11-26T20:37:20.6173365Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hb0517212f4a7a6e1E (wasm-function[42]:8)
2019-11-26T20:37:20.6173846Z     at _ZN3std9panicking3try7do_call17h5f71630b133d918eE (wasm-function[51]:20)
2019-11-26T20:37:20.6174200Z     at __rust_maybe_catch_panic (wasm-function[63]:5)
2019-11-26T20:37:20.6174554Z     at _ZN3std2rt19lang_start_internal17h83f04431ca309805E (wasm-function[60]:250)
2019-11-26T20:37:20.6174826Z     at main (wasm-function[4]:46)
2019-11-26T20:37:20.6175150Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-26T20:37:20.6175258Z     at Module._compile (module.js:641:30)
2019-11-26T20:37:20.6179861Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-26T20:37:20.6179967Z     at Module.load (module.js:560:32)
2019-11-26T20:37:20.6180040Z     at tryModuleLoad (module.js:503:12)
2019-11-26T20:37:20.6180136Z     at Function.Module._load (module.js:495:3)
2019-11-26T20:37:20.6180608Z ------------------------------------------
2019-11-26T20:37:20.6180663Z 
2019-11-26T20:37:20.6180698Z 
2019-11-26T20:37:20.6180750Z 
---
2019-11-26T20:37:20.6182626Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-26T20:37:20.6182736Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-26T20:37:20.6182811Z 
2019-11-26T20:37:20.6182847Z 
2019-11-26T20:37:20.6249244Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-26T20:37:20.6250005Z 
2019-11-26T20:37:20.6250049Z 
2019-11-26T20:37:20.6250699Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-11-26T20:37:20.6250851Z Build completed unsuccessfully in 1:22:06
2019-11-26T20:37:20.6250851Z Build completed unsuccessfully in 1:22:06
2019-11-26T20:37:20.6256371Z == clock drift check ==
2019-11-26T20:37:20.6277976Z   local time: Tue Nov 26 20:37:20 UTC 2019
2019-11-26T20:37:20.7805849Z   network time: Tue, 26 Nov 2019 20:37:20 GMT
2019-11-26T20:37:20.7810563Z == end clock drift check ==
2019-11-26T20:37:22.5508774Z 
2019-11-26T20:37:22.5640512Z ##[error]Bash exited with code '1'.
2019-11-26T20:37:22.5700804Z ##[section]Starting: Checkout
2019-11-26T20:37:22.5704446Z ==============================================================================
2019-11-26T20:37:22.5704569Z Task         : Get sources
2019-11-26T20:37:22.5704655Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
