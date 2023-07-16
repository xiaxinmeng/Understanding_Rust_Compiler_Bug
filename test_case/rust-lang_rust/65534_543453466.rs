plain
2019-10-18T02:01:30.7474087Z test [ui] ui/coherence/coherence_local_err_tuple.rs#re ... ok
2019-10-18T02:01:30.7830188Z test [ui] ui/coherence/coherence_local_ref.rs#old ... ok
2019-10-18T02:01:30.8312594Z test [ui] ui/coherence/conflicting-impl-with-err.rs ... ok
2019-10-18T02:01:30.8431844Z test [ui] ui/coherence/coherence_local_ref.rs#re ... ok
2019-10-18T02:01:30.9343756Z test [ui] ui/coherence/impl-foreign-for-foreign.rs ... ok
2019-10-18T02:01:30.9388695Z test [ui] ui/coherence/impl-foreign-for-foreign[foreign].rs ... ok
2019-10-18T02:01:31.0355027Z test [ui] ui/coherence/impl-foreign-for-fundamental[foreign].rs ... ok
2019-10-18T02:01:31.0396833Z test [ui] ui/coherence/impl-foreign-for-foreign[local].rs ... ok
2019-10-18T02:01:31.1386638Z test [ui] ui/coherence/impl-foreign-for-fundamental[local].rs ... ok
2019-10-18T02:01:31.1512297Z test [ui] ui/coherence/impl-foreign-for-local.rs ... ok
2019-10-18T02:01:31.2467270Z test [ui] ui/coherence/impl-foreign[foreign]-for-local.rs ... ok
2019-10-18T02:01:31.2467270Z test [ui] ui/coherence/impl-foreign[foreign]-for-local.rs ... ok
2019-10-18T02:01:31.3370985Z test [ui] ui/coherence/impl-foreign[fundemental[foreign]]-for-foreign.rs ... ok
2019-10-18T02:01:31.3464695Z test [ui] ui/coherence/impl-foreign[fundemental[local]]-for-foreign.rs ... ok
2019-10-18T02:01:31.4400789Z test [ui] ui/coherence/impl[t]-foreign-for-foreign[t].rs ... ok
2019-10-18T02:01:31.4413151Z test [ui] ui/coherence/impl[t]-foreign-for-(local, t).rs ... ok
2019-10-18T02:01:31.5575529Z test [ui] ui/coherence/impl[t]-foreign-for-fundamental[t].rs ... ok
2019-10-18T02:01:31.6505266Z test [ui] ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs ... ok
2019-10-18T02:01:31.6533086Z test [ui] ui/coherence/impl[t]-foreign[foreign]-for-t.rs ... ok
2019-10-18T02:01:31.7528642Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs ... ok
2019-10-18T02:01:31.7528642Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs ... ok
2019-10-18T02:01:31.7539205Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign.rs ... ok
2019-10-18T02:01:31.8525338Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-local.rs ... ok
2019-10-18T02:01:31.8531665Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-t.rs ... ok
2019-10-18T02:01:31.9521677Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]_local]-for-foreign.rs ... ok
2019-10-18T02:01:31.9538954Z test [ui] ui/coherence/impl[t]-foreign[fundemental[local]]-for-foreign[t].rs ... ok
2019-10-18T02:01:32.0660345Z test [ui] ui/coherence/impl[t]-foreign[local]-for-foreign[t].rs ... ok
2019-10-18T02:01:32.0660345Z test [ui] ui/coherence/impl[t]-foreign[local]-for-foreign[t].rs ... ok
2019-10-18T02:01:32.1656633Z test [ui] ui/coherence/impl[t]-foreign[local]-for-fundamental[foreign[t]].rs ... ok
2019-10-18T02:01:32.2687563Z test [ui] ui/coherence/impl[t]-foreign[local]-for-local.rs ... ok
2019-10-18T02:01:32.2758570Z test [ui] ui/coherence/impl[t]-foreign[local]-for-t.rs ... ok
2019-10-18T02:01:32.3698874Z test [ui] ui/coherence/impl[t]-foreign[t]-for-foreign.rs ... ok
2019-10-18T02:01:32.3727910Z test [ui] ui/coherence/impl[t]-foreign[local_fundamental[t]]-for-foreign.rs ... ok
---
2019-10-18T02:12:17.7612627Z ---- [ui] ui/consts/const-eval/write-to-uninhabited-enum-variant.rs stdout ----
2019-10-18T02:12:17.7612702Z 
2019-10-18T02:12:17.7613012Z error: test run failed!
2019-10-18T02:12:17.7613084Z status: exit code: 101
2019-10-18T02:12:17.7613618Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/a.wasm"
2019-10-18T02:12:17.7614022Z ------------------------------------------
2019-10-18T02:12:17.7614090Z 
2019-10-18T02:12:17.7614325Z ------------------------------------------
2019-10-18T02:12:17.7614414Z stderr:
2019-10-18T02:12:17.7614414Z stderr:
2019-10-18T02:12:17.7614660Z ------------------------------------------
2019-10-18T02:12:17.7614755Z RuntimeError: unreachable
2019-10-18T02:12:17.7615043Z     at _ZN3std7process4exit17h328f88c59e36dc5cE (wasm-function[41]:7)
2019-10-18T02:12:17.7615385Z     at _ZN33write_to_uninhabited_enum_variant3bar17h017db0f6c0883816E (wasm-function[3]:3)
2019-10-18T02:12:17.7615731Z     at _ZN33write_to_uninhabited_enum_variant4main17hced8f1a3a6cc2e0bE (wasm-function[4]:1)
2019-10-18T02:12:17.7616084Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hdb39e8fceb70f453E (wasm-function[0]:25)
2019-10-18T02:12:17.7616459Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h1ed57b0f73acd440E (wasm-function[45]:8)
2019-10-18T02:12:17.7616780Z     at _ZN3std9panicking3try7do_call17h501f0d80d0bbf033E (wasm-function[54]:20)
2019-10-18T02:12:17.7617078Z     at __rust_maybe_catch_panic (wasm-function[66]:5)
2019-10-18T02:12:17.7617401Z     at _ZN3std2rt19lang_start_internal17h3561a1458b6fa0ccE (wasm-function[63]:250)
2019-10-18T02:12:17.7617784Z     at main (wasm-function[5]:46)
2019-10-18T02:12:17.7618126Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-18T02:12:17.7618211Z     at Module._compile (module.js:641:30)
2019-10-18T02:12:17.7618308Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-18T02:12:17.7618385Z     at Module.load (module.js:560:32)
2019-10-18T02:12:17.7618474Z     at tryModuleLoad (module.js:503:12)
2019-10-18T02:12:17.7618579Z     at Function.Module._load (module.js:495:3)
2019-10-18T02:12:17.7618660Z     at Function.Module.runMain (module.js:682:10)
2019-10-18T02:12:17.7618753Z     at startup (bootstrap_node.js:191:16)
2019-10-18T02:12:17.7618826Z     at bootstrap_node.js:613:3
2019-10-18T02:12:17.7619131Z ------------------------------------------
2019-10-18T02:12:17.7619184Z 
2019-10-18T02:12:17.7619236Z 
2019-10-18T02:12:17.7619270Z 
---
2019-10-18T02:12:17.7653675Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-18T02:12:17.7653816Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-18T02:12:17.7669921Z 
2019-10-18T02:12:17.7670008Z 
2019-10-18T02:12:17.7675814Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-18T02:12:17.7676793Z 
2019-10-18T02:12:17.7676835Z 
2019-10-18T02:12:17.7689721Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-10-18T02:12:17.7689905Z Build completed unsuccessfully in 1:24:18
2019-10-18T02:12:17.7689905Z Build completed unsuccessfully in 1:24:18
2019-10-18T02:12:17.7746127Z == clock drift check ==
2019-10-18T02:12:17.7763854Z   local time: Fri Oct 18 02:12:17 UTC 2019
2019-10-18T02:12:18.0569918Z   network time: Fri, 18 Oct 2019 02:12:18 GMT
2019-10-18T02:12:18.0573055Z == end clock drift check ==
2019-10-18T02:12:19.2050635Z ##[error]Bash exited with code '1'.
2019-10-18T02:12:19.2089019Z ##[section]Starting: Upload CPU usage statistics
2019-10-18T02:12:19.2099166Z ==============================================================================
2019-10-18T02:12:19.2099277Z Task         : Bash
2019-10-18T02:12:19.2099379Z Description  : Run a Bash script on macOS, Linux, or Windows
