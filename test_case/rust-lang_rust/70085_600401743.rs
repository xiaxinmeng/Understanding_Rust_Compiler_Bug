plain
2020-03-18T03:07:39.4767048Z ---- [ui] ui/eprint-on-tls-drop.rs stdout ----
2020-03-18T03:07:39.4767576Z 
2020-03-18T03:07:39.4767896Z error: test run failed!
2020-03-18T03:07:39.4768417Z status: exit code: 101
2020-03-18T03:07:39.4769671Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/eprint-on-tls-drop/a.wasm"
2020-03-18T03:07:39.4771637Z ------------------------------------------
2020-03-18T03:07:39.4772344Z 
2020-03-18T03:07:39.4772839Z ------------------------------------------
2020-03-18T03:07:39.4773526Z stderr:
2020-03-18T03:07:39.4773526Z stderr:
2020-03-18T03:07:39.4774043Z ------------------------------------------
2020-03-18T03:07:39.4774418Z RuntimeError: unreachable
2020-03-18T03:07:39.4774986Z     at __rust_start_panic (wasm-function[102]:1)
2020-03-18T03:07:39.4775670Z     at rust_panic (wasm-function[97]:39)
2020-03-18T03:07:39.4776600Z     at _ZN3std9panicking20rust_panic_with_hook17h3776c63be80274d9E (wasm-function[92]:279)
2020-03-18T03:07:39.4777424Z     at rust_begin_unwind (wasm-function[91]:90)
2020-03-18T03:07:39.4778188Z     at _ZN4core9panicking9panic_fmt17hfe91412ce4db60bcE (wasm-function[121]:58)
2020-03-18T03:07:39.4779042Z     at _ZN4core6option18expect_none_failed17h09d6e81dcbb6691aE (wasm-function[133]:139)
2020-03-18T03:07:39.4779903Z     at _ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h4610508555dadc7fE (wasm-function[1]:67)
2020-03-18T03:07:39.4780756Z     at _ZN18eprint_on_tls_drop4main17he6677df84c8c9a17E (wasm-function[3]:106)
2020-03-18T03:07:39.4781651Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h000229a249a1659fE (wasm-function[5]:25)
2020-03-18T03:07:39.4782606Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he3b7505725498491E (wasm-function[82]:8)
2020-03-18T03:07:39.4783518Z     at _ZN3std2rt19lang_start_internal17h7d8e6b2ac3a24e45E (wasm-function[98]:285)
2020-03-18T03:07:39.4784225Z     at main (wasm-function[4]:46)
2020-03-18T03:07:39.4785043Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2020-03-18T03:07:39.4785518Z     at Module._compile (module.js:641:30)
2020-03-18T03:07:39.4785958Z     at Object.Module._extensions..js (module.js:652:10)
2020-03-18T03:07:39.4786359Z     at Module.load (module.js:560:32)
2020-03-18T03:07:39.4786971Z     at tryModuleLoad (module.js:503:12)
2020-03-18T03:07:39.4787754Z     at Function.Module._load (module.js:495:3)
2020-03-18T03:07:39.4788194Z     at Function.Module.runMain (module.js:682:10)
2020-03-18T03:07:39.4788602Z     at startup (bootstrap_node.js:191:16)
2020-03-18T03:07:39.4789868Z ------------------------------------------
2020-03-18T03:07:39.4790159Z 
2020-03-18T03:07:39.4790376Z 
2020-03-18T03:07:39.4790857Z ---- [ui] ui/panic-while-printing.rs stdout ----
2020-03-18T03:07:39.4790857Z ---- [ui] ui/panic-while-printing.rs stdout ----
2020-03-18T03:07:39.4791184Z 
2020-03-18T03:07:39.4791470Z error: test run failed!
2020-03-18T03:07:39.4791779Z status: exit code: 101
2020-03-18T03:07:39.4792601Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-while-printing/a.wasm"
2020-03-18T03:07:39.4796034Z ------------------------------------------
2020-03-18T03:07:39.4796203Z 
2020-03-18T03:07:39.4796552Z ------------------------------------------
2020-03-18T03:07:39.4796767Z stderr:
2020-03-18T03:07:39.4796767Z stderr:
2020-03-18T03:07:39.4797105Z ------------------------------------------
2020-03-18T03:07:39.4797352Z RuntimeError: unreachable
2020-03-18T03:07:39.4797773Z     at __rust_start_panic (wasm-function[111]:1)
2020-03-18T03:07:39.4798233Z     at rust_panic (wasm-function[107]:39)
2020-03-18T03:07:39.4798815Z     at _ZN3std9panicking20rust_panic_with_hook17h3776c63be80274d9E (wasm-function[102]:279)
2020-03-18T03:07:39.4799484Z     at _ZN3std9panicking11begin_panic17hc505a39c90486efdE (wasm-function[12]:55)
2020-03-18T03:07:39.4800475Z     at _ZN62_$LT$panic_while_printing..A$u20$as$u20$core..fmt..Display$GT$3fmt17h7c1603593332d80dE (wasm-function[29]:15)
2020-03-18T03:07:39.4801528Z     at _ZN4core3fmt5write17h9d09f709a91fc8b5E (wasm-function[132]:713)
2020-03-18T03:07:39.4802147Z     at _ZN3std2io5Write9write_fmt17h14444a7b7a6e7e63E (wasm-function[2]:104)
2020-03-18T03:07:39.4802757Z     at _ZN3std2io5stdio7_eprint17h55276cb39b8308feE (wasm-function[88]:438)
2020-03-18T03:07:39.4803673Z     at _ZN20panic_while_printing4main17he42181185949748fE (wasm-function[30]:192)
2020-03-18T03:07:39.4804565Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h0f2f34847fba8a98E (wasm-function[32]:25)
2020-03-18T03:07:39.4805319Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he3b7505725498491E (wasm-function[93]:8)
2020-03-18T03:07:39.4806007Z     at _ZN3std2rt19lang_start_internal17h7d8e6b2ac3a24e45E (wasm-function[108]:285)
2020-03-18T03:07:39.4806677Z     at main (wasm-function[31]:46)
2020-03-18T03:07:39.4807434Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2020-03-18T03:07:39.4807760Z     at Module._compile (module.js:641:30)
2020-03-18T03:07:39.4808082Z     at Object.Module._extensions..js (module.js:652:10)
2020-03-18T03:07:39.4808575Z     at Module.load (module.js:560:32)
2020-03-18T03:07:39.4813015Z     at tryModuleLoad (module.js:503:12)
2020-03-18T03:07:39.4813371Z     at Function.Module._load (module.js:495:3)
2020-03-18T03:07:39.4813673Z     at Function.Module.runMain (module.js:682:10)
2020-03-18T03:07:39.4814331Z ------------------------------------------
2020-03-18T03:07:39.4814491Z 
2020-03-18T03:07:39.4814599Z 
2020-03-18T03:07:39.4815119Z ---- [ui] ui/test-panic-while-printing.rs stdout ----
2020-03-18T03:07:39.4815119Z ---- [ui] ui/test-panic-while-printing.rs stdout ----
2020-03-18T03:07:39.4815303Z 
2020-03-18T03:07:39.4815479Z error: test run failed!
2020-03-18T03:07:39.4815699Z status: exit code: 101
2020-03-18T03:07:39.4816402Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-while-printing/a.wasm"
2020-03-18T03:07:39.4817180Z ------------------------------------------
2020-03-18T03:07:39.4817360Z 
2020-03-18T03:07:39.4817689Z ------------------------------------------
2020-03-18T03:07:39.4817903Z stderr:
2020-03-18T03:07:39.4817903Z stderr:
2020-03-18T03:07:39.4818238Z ------------------------------------------
2020-03-18T03:07:39.4818834Z RuntimeError: unreachable
2020-03-18T03:07:39.4819439Z     at __rust_start_panic (wasm-function[438]:1)
2020-03-18T03:07:39.4819932Z     at rust_panic (wasm-function[427]:39)
2020-03-18T03:07:39.4821283Z     at _ZN3std9panicking20rust_panic_with_hook17h3776c63be80274d9E (wasm-function[422]:279)
2020-03-18T03:07:39.4821885Z     at rust_begin_unwind (wasm-function[421]:90)
2020-03-18T03:07:39.4822626Z     at _ZN4core9panicking9panic_fmt17hfe91412ce4db60bcE (wasm-function[475]:58)
2020-03-18T03:07:39.4823285Z     at _ZN4core9panicking18panic_bounds_check17h9f215058f990ce74E (wasm-function[467]:121)
2020-03-18T03:07:39.4824095Z     at _ZN67_$LT$test_panic_while_printing..A$u20$as$u20$core..fmt..Display$GT$3fmt17h8ae97b01f42810beE (wasm-function[2]:20)
2020-03-18T03:07:39.4824798Z     at _ZN4core3fmt5write17h9d09f709a91fc8b5E (wasm-function[485]:713)
2020-03-18T03:07:39.4825421Z     at _ZN3std2io5Write9write_fmt17h2cf7e6ae8345d761E (wasm-function[29]:104)
2020-03-18T03:07:39.4826044Z     at _ZN3std2io5stdio7_eprint17h55276cb39b8308feE (wasm-function[382]:438)
2020-03-18T03:07:39.4826686Z     at _ZN4core3ops8function6FnOnce9call_once17h887455bd155ff194E (wasm-function[1]:95)
2020-03-18T03:07:39.4827366Z     at _ZN4test28__rust_begin_short_backtrace17h77e94495f51ad351E (wasm-function[70]:3)
2020-03-18T03:07:39.4828130Z     at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h95981923f27bf1d7E (wasm-function[69]:6)
2020-03-18T03:07:39.4829473Z     at _ZN83_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$9call_once17h640c76ef9e37d2e6E (wasm-function[127]:54)
2020-03-18T03:07:39.4830467Z     at _ZN4test8run_test14run_test_inner17h2b56e423f4aa8c8fE (wasm-function[185]:2040)
2020-03-18T03:07:39.4831089Z     at _ZN4test8run_test17h40ad132414fcc056E (wasm-function[182]:812)
2020-03-18T03:07:39.4832074Z     at _ZN4test9run_tests17h7ca6c30363716732E (wasm-function[138]:17379)
2020-03-18T03:07:39.4832712Z     at _ZN4test7console17run_tests_console17h2296c03aeb9b53ceE (wasm-function[136]:1014)
2020-03-18T03:07:39.4833347Z     at _ZN4test9test_main17he3c08b51ea9e86caE (wasm-function[178]:342)
2020-03-18T03:07:39.4834134Z     at _ZN4test16test_main_static17hf1dd3e1210be7304E (wasm-function[180]:367)
2020-03-18T03:07:39.4834762Z ------------------------------------------
2020-03-18T03:07:39.4834949Z 
2020-03-18T03:07:39.4835046Z 
2020-03-18T03:07:39.4835142Z 
---
2020-03-18T03:07:39.4837963Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T03:07:39.4838386Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T03:07:39.4838877Z 
2020-03-18T03:07:39.4886485Z 
2020-03-18T03:07:39.4891198Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.44.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T03:07:39.4894504Z 
2020-03-18T03:07:39.4894601Z 
2020-03-18T03:07:39.4895579Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-03-18T03:07:39.4896666Z Build completed unsuccessfully in 1:29:41
2020-03-18T03:07:39.4896666Z Build completed unsuccessfully in 1:29:41
2020-03-18T03:07:39.4903820Z == clock drift check ==
2020-03-18T03:07:39.4924970Z   local time: Wed Mar 18 03:07:39 UTC 2020
2020-03-18T03:07:39.7645391Z   network time: Wed, 18 Mar 2020 03:07:39 GMT
2020-03-18T03:07:39.7650095Z == end clock drift check ==
2020-03-18T03:07:40.9356956Z 
2020-03-18T03:07:40.9395273Z ##[error]Bash exited with code '1'.
2020-03-18T03:07:40.9475924Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-18T03:07:40.9481223Z ==============================================================================
2020-03-18T03:07:40.9481691Z Task         : Get sources
2020-03-18T03:07:40.9482159Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
