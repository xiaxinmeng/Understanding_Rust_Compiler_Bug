plain
2020-03-14T06:47:25.2975201Z test [ui] ui/env-vars.rs ... ignored
2020-03-14T06:47:25.3109386Z test [ui] ui/enums-pats-not-idents.rs ... ok
2020-03-14T06:47:25.5601304Z test [ui] ui/epoch-gate-feature.rs ... ok
2020-03-14T06:47:25.9768506Z test [ui] ui/eq-multidispatch.rs ... ok
2020-03-14T06:47:25.9790785Z test [ui] ui/eprint-on-tls-drop.rs ... FAILED
2020-03-14T06:47:26.0188302Z test [ui] ui/error-codes/E0004-2.rs ... ok
2020-03-14T06:47:26.0712250Z test [ui] ui/error-codes/E0005.rs ... ok
2020-03-14T06:47:26.0730068Z test [ui] ui/error-codes/E0004.rs ... ok
2020-03-14T06:47:26.1105214Z test [ui] ui/error-codes/E0010-teach.rs ... ok
---
2020-03-14T06:57:55.8631657Z test [ui] ui/wrapping-int-combinations.rs ... ok
2020-03-14T06:57:55.8635736Z 
2020-03-14T06:57:55.8636031Z failures:
2020-03-14T06:57:55.8666048Z 
2020-03-14T06:57:55.8666710Z ---- [ui] ui/eprint-on-tls-drop.rs stdout ----
2020-03-14T06:57:55.8667145Z error: test run failed!
2020-03-14T06:57:55.8667369Z status: exit code: 101
2020-03-14T06:57:55.8667369Z status: exit code: 101
2020-03-14T06:57:55.8668119Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/eprint-on-tls-drop/a.wasm"
2020-03-14T06:57:55.8668931Z ------------------------------------------
2020-03-14T06:57:55.8669105Z 
2020-03-14T06:57:55.8669467Z ------------------------------------------
2020-03-14T06:57:55.8669694Z stderr:
2020-03-14T06:57:55.8669694Z stderr:
2020-03-14T06:57:55.8670051Z ------------------------------------------
2020-03-14T06:57:55.8670312Z RuntimeError: unreachable
2020-03-14T06:57:55.8670955Z     at __rust_start_panic (wasm-function[102]:1)
2020-03-14T06:57:55.8671461Z     at rust_panic (wasm-function[97]:39)
2020-03-14T06:57:55.8672100Z     at _ZN3std9panicking20rust_panic_with_hook17hce8da6fb7d79ed0eE (wasm-function[92]:279)
2020-03-14T06:57:55.8672728Z     at rust_begin_unwind (wasm-function[91]:90)
2020-03-14T06:57:55.8673352Z     at _ZN4core9panicking9panic_fmt17h0843d8e9cd281e9dE (wasm-function[121]:58)
2020-03-14T06:57:55.8674181Z     at _ZN4core6option18expect_none_failed17h800de8fcb4c10b2bE (wasm-function[133]:139)
2020-03-14T06:57:55.8675023Z     at _ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hce76bed18edc3eafE (wasm-function[2]:67)
2020-03-14T06:57:55.8675859Z     at _ZN18eprint_on_tls_drop4main17he6677df84c8c9a17E (wasm-function[3]:106)
2020-03-14T06:57:55.8676769Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e66e6d369820aeE (wasm-function[5]:25)
2020-03-14T06:57:55.8677659Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha5a6714d3fad5170E (wasm-function[82]:8)
2020-03-14T06:57:55.8678489Z     at _ZN3std2rt19lang_start_internal17h51f54df58857afefE (wasm-function[98]:285)
2020-03-14T06:57:55.8679092Z     at main (wasm-function[4]:46)
2020-03-14T06:57:55.8679662Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2020-03-14T06:57:55.8680064Z     at Module._compile (module.js:641:30)
2020-03-14T06:57:55.8680448Z     at Object.Module._extensions..js (module.js:652:10)
2020-03-14T06:57:55.8680823Z     at Module.load (module.js:560:32)
2020-03-14T06:57:55.8681146Z     at tryModuleLoad (module.js:503:12)
2020-03-14T06:57:55.8681709Z     at Function.Module._load (module.js:495:3)
2020-03-14T06:57:55.8682278Z     at Function.Module.runMain (module.js:682:10)
2020-03-14T06:57:55.8682655Z     at startup (bootstrap_node.js:191:16)
2020-03-14T06:57:55.8683593Z ------------------------------------------
2020-03-14T06:57:55.8683777Z 
2020-03-14T06:57:55.8683880Z 
2020-03-14T06:57:55.8684004Z 
2020-03-14T06:57:55.8684004Z 
2020-03-14T06:57:55.8684153Z failures:
2020-03-14T06:57:55.8684769Z     [ui] ui/eprint-on-tls-drop.rs
2020-03-14T06:57:55.8686035Z test result: FAILED. 9385 passed; 1 failed; 386 ignored; 0 measured; 0 filtered out
2020-03-14T06:57:55.8686327Z 
2020-03-14T06:57:55.8713853Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-14T06:57:55.8714320Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T06:57:55.8714320Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T06:57:55.8727481Z 
2020-03-14T06:57:55.8727668Z 
2020-03-14T06:57:55.8736787Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-14T06:57:55.8740526Z 
2020-03-14T06:57:55.8740743Z 
2020-03-14T06:57:55.8755049Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-03-14T06:57:55.8755728Z Build completed unsuccessfully in 1:37:39
2020-03-14T06:57:55.8755728Z Build completed unsuccessfully in 1:37:39
2020-03-14T06:57:55.8822169Z == clock drift check ==
2020-03-14T06:57:55.8843249Z   local time: Sat Mar 14 06:57:55 UTC 2020
2020-03-14T06:57:56.0462563Z   network time: Sat, 14 Mar 2020 06:57:56 GMT
2020-03-14T06:57:56.0463664Z == end clock drift check ==
2020-03-14T06:57:57.1849094Z 
2020-03-14T06:57:57.1957306Z ##[error]Bash exited with code '1'.
2020-03-14T06:57:57.2037504Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-14T06:57:57.2043281Z ==============================================================================
2020-03-14T06:57:57.2043636Z Task         : Get sources
2020-03-14T06:57:57.2043979Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
