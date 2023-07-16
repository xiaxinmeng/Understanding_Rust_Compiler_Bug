plain
2020-02-18T22:27:08.5386670Z failures:
2020-02-18T22:27:08.5417746Z 
2020-02-18T22:27:08.5418831Z ---- [ui] ui/issues/issue-69225-layout-repeated-checked-add.rs stdout ----
2020-02-18T22:27:08.5419146Z 
2020-02-18T22:27:08.5419720Z error: error pattern ' index out of bounds: the len is 0 but the index is 16777216' not found!
2020-02-18T22:27:08.5420583Z status: exit code: 101
2020-02-18T22:27:08.5421215Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69225-layout-repeated-checked-add/a.wasm"
2020-02-18T22:27:08.5422014Z ------------------------------------------
2020-02-18T22:27:08.5422281Z 
2020-02-18T22:27:08.5422714Z ------------------------------------------
2020-02-18T22:27:08.5423008Z stderr:
2020-02-18T22:27:08.5423008Z stderr:
2020-02-18T22:27:08.5423589Z ------------------------------------------
2020-02-18T22:27:08.5423885Z RuntimeError: unreachable
2020-02-18T22:27:08.5424338Z     at __rust_start_panic (wasm-function[86]:1)
2020-02-18T22:27:08.5424816Z     at rust_panic (wasm-function[81]:39)
2020-02-18T22:27:08.5425365Z     at _ZN3std9panicking20rust_panic_with_hook17haccf39a209840b13E (wasm-function[76]:279)
2020-02-18T22:27:08.5425850Z     at rust_begin_unwind (wasm-function[75]:90)
2020-02-18T22:27:08.5426390Z     at _ZN4core9panicking9panic_fmt17he40e0f446bee653cE (wasm-function[105]:58)
2020-02-18T22:27:08.5426947Z     at _ZN4core9panicking18panic_bounds_check17h8dd5e3acacef9845E (wasm-function[99]:121)
2020-02-18T22:27:08.5427494Z     at _ZN39issue_69225_layout_repeated_checked_add7do_test17h5404806e7149312bE (wasm-function[5]:622)
2020-02-18T22:27:08.5428269Z     at _ZN39issue_69225_layout_repeated_checked_add4main17h44f1bf96cc64fc61E (wasm-function[6]:3)
2020-02-18T22:27:08.5428852Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf331ae495ada9e50E (wasm-function[0]:25)
2020-02-18T22:27:08.5429421Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h820edc2dd482f4e1E (wasm-function[65]:8)
2020-02-18T22:27:08.5430246Z     at _ZN3std9panicking3try7do_call17hf43b5cc4585ba399E (wasm-function[74]:20)
2020-02-18T22:27:08.5430871Z     at __rust_maybe_catch_panic (wasm-function[85]:5)
2020-02-18T22:27:08.5431684Z     at _ZN3std2rt19lang_start_internal17hd2e1ca2709777a41E (wasm-function[82]:250)
2020-02-18T22:27:08.5432351Z     at main (wasm-function[7]:46)
2020-02-18T22:27:08.5432857Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2020-02-18T22:27:08.5433163Z     at Module._compile (module.js:641:30)
2020-02-18T22:27:08.5433420Z     at Object.Module._extensions..js (module.js:652:10)
2020-02-18T22:27:08.5433658Z     at Module.load (module.js:560:32)
2020-02-18T22:27:08.5433902Z     at tryModuleLoad (module.js:503:12)
2020-02-18T22:27:08.5434136Z     at Function.Module._load (module.js:495:3)
2020-02-18T22:27:08.5434784Z ------------------------------------------
2020-02-18T22:27:08.5435208Z 
2020-02-18T22:27:08.5435401Z 
2020-02-18T22:27:08.5435582Z 
---
2020-02-18T22:27:08.5450807Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-18T22:27:08.5451592Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-18T22:27:08.5468622Z 
2020-02-18T22:27:08.5468782Z 
2020-02-18T22:27:08.5470875Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-18T22:27:08.5472141Z 
2020-02-18T22:27:08.5472176Z 
2020-02-18T22:27:08.5480671Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-02-18T22:27:08.5481055Z Build completed unsuccessfully in 1:26:14
2020-02-18T22:27:08.5481055Z Build completed unsuccessfully in 1:26:14
2020-02-18T22:27:08.5546768Z == clock drift check ==
2020-02-18T22:27:08.5565813Z   local time: Tue Feb 18 22:27:08 UTC 2020
2020-02-18T22:27:08.8509857Z   network time: Tue, 18 Feb 2020 22:27:08 GMT
2020-02-18T22:27:08.8513177Z == end clock drift check ==
2020-02-18T22:27:09.6885300Z 
2020-02-18T22:27:09.6943990Z ##[error]Bash exited with code '1'.
2020-02-18T22:27:09.6985869Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-18T22:27:09.6988022Z ==============================================================================
2020-02-18T22:27:09.6988115Z Task         : Get sources
2020-02-18T22:27:09.6988324Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
