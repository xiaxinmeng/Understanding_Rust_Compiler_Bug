plain
2020-03-15T00:38:05.3417589Z ---- [ui] ui/panic-while-printing.rs stdout ----
2020-03-15T00:38:05.3417841Z 
2020-03-15T00:38:05.3417985Z error: test run failed!
2020-03-15T00:38:05.3418201Z status: exit code: 101
2020-03-15T00:38:05.3418843Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-while-printing/a.wasm"
2020-03-15T00:38:05.3419504Z ------------------------------------------
2020-03-15T00:38:05.3419648Z 
2020-03-15T00:38:05.3419952Z ------------------------------------------
2020-03-15T00:38:05.3420124Z stderr:
2020-03-15T00:38:05.3420124Z stderr:
2020-03-15T00:38:05.3420431Z ------------------------------------------
2020-03-15T00:38:05.3420632Z RuntimeError: unreachable
2020-03-15T00:38:05.3421009Z     at __rust_start_panic (wasm-function[113]:1)
2020-03-15T00:38:05.3421385Z     at rust_panic (wasm-function[109]:39)
2020-03-15T00:38:05.3421898Z     at _ZN3std9panicking20rust_panic_with_hook17h36069e578eb036a4E (wasm-function[104]:279)
2020-03-15T00:38:05.3422478Z     at _ZN3std9panicking11begin_panic17h4666bbf17dd74484E (wasm-function[12]:55)
2020-03-15T00:38:05.3423335Z     at _ZN62_$LT$panic_while_printing..A$u20$as$u20$core..fmt..Display$GT$3fmt17h3690c5f3330e14d3E (wasm-function[29]:15)
2020-03-15T00:38:05.3423979Z     at _ZN4core3fmt5write17h3b13085a45c0f3c9E (wasm-function[134]:713)
2020-03-15T00:38:05.3424519Z     at _ZN3std2io5Write9write_fmt17hf996630c9ed61762E (wasm-function[2]:104)
2020-03-15T00:38:05.3425087Z     at _ZN3std2io5stdio7_eprint17hde89c4053d75c8fbE (wasm-function[90]:416)
2020-03-15T00:38:05.3425670Z     at _ZN20panic_while_printing4main17he42181185949748fE (wasm-function[30]:192)
2020-03-15T00:38:05.3426309Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hac92c8d7cc816a53E (wasm-function[32]:25)
2020-03-15T00:38:05.3427003Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h7aeaec061595e007E (wasm-function[95]:8)
2020-03-15T00:38:05.3427632Z     at _ZN3std2rt19lang_start_internal17h672fef26699e9019E (wasm-function[110]:285)
2020-03-15T00:38:05.3428099Z     at main (wasm-function[31]:46)
2020-03-15T00:38:05.3428955Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2020-03-15T00:38:05.3429253Z     at Module._compile (module.js:641:30)
2020-03-15T00:38:05.3429545Z     at Object.Module._extensions..js (module.js:652:10)
2020-03-15T00:38:05.3429804Z     at Module.load (module.js:560:32)
2020-03-15T00:38:05.3430053Z     at tryModuleLoad (module.js:503:12)
2020-03-15T00:38:05.3430303Z     at Function.Module._load (module.js:495:3)
2020-03-15T00:38:05.3430696Z     at Function.Module.runMain (module.js:682:10)
2020-03-15T00:38:05.3431213Z ------------------------------------------
2020-03-15T00:38:05.3431356Z 
2020-03-15T00:38:05.3431438Z 
2020-03-15T00:38:05.3431768Z ---- [ui] ui/test-panic-while-printing.rs stdout ----
2020-03-15T00:38:05.3431768Z ---- [ui] ui/test-panic-while-printing.rs stdout ----
2020-03-15T00:38:05.3431926Z 
2020-03-15T00:38:05.3432076Z error: test run failed!
2020-03-15T00:38:05.3432250Z status: exit code: 101
2020-03-15T00:38:05.3432859Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-while-printing/a.wasm"
2020-03-15T00:38:05.3433526Z ------------------------------------------
2020-03-15T00:38:05.3433669Z 
2020-03-15T00:38:05.3433966Z ------------------------------------------
2020-03-15T00:38:05.3434138Z stderr:
2020-03-15T00:38:05.3434138Z stderr:
2020-03-15T00:38:05.3434441Z ------------------------------------------
2020-03-15T00:38:05.3434657Z RuntimeError: unreachable
2020-03-15T00:38:05.3435021Z     at __rust_start_panic (wasm-function[439]:1)
2020-03-15T00:38:05.3435421Z     at rust_panic (wasm-function[428]:39)
2020-03-15T00:38:05.3435915Z     at _ZN3std9panicking20rust_panic_with_hook17h36069e578eb036a4E (wasm-function[423]:279)
2020-03-15T00:38:05.3436407Z     at rust_begin_unwind (wasm-function[422]:90)
2020-03-15T00:38:05.3437073Z     at _ZN4core9panicking9panic_fmt17ha9734a4de0d5b1f3E (wasm-function[476]:58)
2020-03-15T00:38:05.3437706Z     at _ZN4core9panicking18panic_bounds_check17h6420ad870c9f5b03E (wasm-function[468]:121)
2020-03-15T00:38:05.3438402Z     at _ZN67_$LT$test_panic_while_printing..A$u20$as$u20$core..fmt..Display$GT$3fmt17h2ab2d3c9652d8d16E (wasm-function[4]:20)
2020-03-15T00:38:05.3439000Z     at _ZN4core3fmt5write17h3b13085a45c0f3c9E (wasm-function[486]:713)
2020-03-15T00:38:05.3439538Z     at _ZN3std2io5Write9write_fmt17h8ca4c539c6caaf76E (wasm-function[29]:104)
2020-03-15T00:38:05.3440065Z     at _ZN3std2io5stdio7_eprint17hde89c4053d75c8fbE (wasm-function[383]:416)
2020-03-15T00:38:05.3440632Z     at _ZN4core3ops8function6FnOnce9call_once17h3bcf99cc30f7579bE (wasm-function[3]:95)
2020-03-15T00:38:05.3441211Z     at _ZN4test28__rust_begin_short_backtrace17h82e6dd566a3b4404E (wasm-function[72]:3)
2020-03-15T00:38:05.3441865Z     at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h6c4819177ecf744eE (wasm-function[71]:6)
2020-03-15T00:38:05.3442667Z     at _ZN83_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$9call_once17hf8c3b0f145074180E (wasm-function[127]:54)
2020-03-15T00:38:05.3443357Z     at _ZN4test8run_test14run_test_inner17h0c08055f3b8d425fE (wasm-function[185]:2040)
2020-03-15T00:38:05.3443879Z     at _ZN4test8run_test17hf981f310bf01c73fE (wasm-function[182]:812)
2020-03-15T00:38:05.3444391Z     at _ZN4test9run_tests17h0cc118f7a246327aE (wasm-function[138]:17379)
2020-03-15T00:38:05.3444935Z     at _ZN4test7console17run_tests_console17h57fc4c8a9466e25bE (wasm-function[136]:1014)
2020-03-15T00:38:05.3445484Z     at _ZN4test9test_main17h15ed570cf887b055E (wasm-function[178]:342)
2020-03-15T00:38:05.3446002Z     at _ZN4test16test_main_static17hf42fd439fc58c3e7E (wasm-function[180]:367)
2020-03-15T00:38:05.3446520Z ------------------------------------------
2020-03-15T00:38:05.3446678Z 
2020-03-15T00:38:05.3446760Z 
2020-03-15T00:38:05.3446840Z 
---
2020-03-15T00:38:05.3469641Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-15T00:38:05.3470227Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-15T00:38:05.3479690Z 
2020-03-15T00:38:05.3479814Z 
2020-03-15T00:38:05.3487570Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-15T00:38:05.3489865Z 
2020-03-15T00:38:05.3489968Z 
2020-03-15T00:38:05.3505512Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-03-15T00:38:05.3506067Z Build completed unsuccessfully in 1:25:34
2020-03-15T00:38:05.3506067Z Build completed unsuccessfully in 1:25:34
2020-03-15T00:38:05.3569617Z == clock drift check ==
2020-03-15T00:38:05.3592629Z   local time: Sun Mar 15 00:38:05 UTC 2020
2020-03-15T00:38:05.8875689Z   network time: Sun, 15 Mar 2020 00:38:05 GMT
2020-03-15T00:38:05.8882722Z == end clock drift check ==
2020-03-15T00:38:06.7049382Z 
2020-03-15T00:38:06.7111964Z ##[error]Bash exited with code '1'.
2020-03-15T00:38:06.7189580Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-15T00:38:06.7196221Z ==============================================================================
2020-03-15T00:38:06.7198317Z Task         : Get sources
2020-03-15T00:38:06.7198940Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
