plain
[00:53:18] test [ui] ui/rfc-2306/convert-id-const-no-gate.rs ... ok
[00:53:18] test [ui] ui/rfc-2126-extern-absolute-paths/single-segment.rs ... ok
[00:53:18] test [ui] ui/rfc-2166-underscore-imports/basic.rs ... ok
[00:53:18] test [ui] ui/rfc-2126-extern-in-paths/single-segment.rs ... ok
[00:53:18] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-feature-gate.rs ... ok
[00:53:18] test [ui] ui/rfc-2306/convert-id-const-with-gate.rs ... ok
[00:53:18] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs ... ok
[00:53:18] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs ... ok
[00:53:18] test [ui] ui/rfc1445/feature-gate.rs#no_gate ... ok
[00:53:18] test [ui] ui/rfc1445/feature-gate.rs#with_gate ... ok
[00:53:18] test [ui] ui/rfc-2497-if-let-chains/syntax-ambiguity-2018.rs ... ok
[00:53:18] test [ui] ui/rfc1445/match-forbidden-without-eq.rs ... ok
---
[00:53:19] test [ui] ui/rfc1717/missing-link-attr.rs ... ok
[00:53:19] test [ui] ui/rfc1717/rename-to-empty.rs ... ok
[00:53:19] test [ui] ui/rfc1717/multiple-renames.rs ... ok
[00:53:19] test [ui] ui/rmeta-priv-warn.rs ... ok
[00:53:19] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs ... FAILED
[00:53:19] test [ui] ui/rmeta.rs ... ok
[00:53:19] test [ui] ui/rmeta-pass.rs ... ok
[00:53:19] test [ui] ui/rmeta_lib.rs ... ok
[00:53:19] test [ui] ui/rmeta_meta_main.rs ... ok
---
[00:58:38] test [ui] ui/run-pass/zero-sized/zero-sized-btreemap-insert.rs ... ok
[00:58:38] test [ui] ui/rust-2018/dyn-trait-compatibility.rs ... ok
[00:58:38] test [ui] ui/run-pass/zero-sized/zero-sized-vec-push.rs ... ok
[00:58:38] test [ui] ui/rust-2018/async-ident.rs ... ok
[00:58:38] test [ui] ui/rust-2018/dyn-keyword.rs ... ok
[00:58:38] test [ui] ui/rust-2018/edition-lint-nested-empty-paths.rs ... ok
[00:58:38] test [ui] ui/rust-2018/edition-lint-nested-paths.rs ... ok
[00:58:39] test [ui] ui/rust-2018/edition-lint-paths.rs ... ok
[00:58:39] test [ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs ... ok
---
[00:59:02] test [ui] ui/xcrate/xcrate-private-by-default.rs ... ok
[00:59:02] 
[00:59:02] failures:
[00:59:02] 
[00:59:02] ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
[00:59:02] error: test run failed!
[00:59:02] status: exit code: 101
[00:59:02] status: exit code: 101
[00:59:02] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a.wasm"
[00:59:02] ------------------------------------------
[00:59:02] 
[00:59:02] ------------------------------------------
[00:59:02] stderr:
[00:59:02] stderr:
[00:59:02] ------------------------------------------
[00:59:02] RuntimeError: unreachable
[00:59:02]     at __rust_start_panic (wasm-function[148]:1)
[00:59:02]     at rust_panic.llvm.7229658430215002035 (wasm-function[142]:38)
[00:59:02]     at std::panicking::rust_panic_with_hook::h226a58a5977f2e17 (wasm-function[137]:500)
[00:59:02]     at std::panicking::continue_panic_fmt::h5f0126bd5d9da93f (wasm-function[136]:155)
[00:59:02]     at rust_begin_unwind (wasm-function[135]:3)
[00:59:02]     at core::panicking::panic_fmt::hebc42c0af7746402 (wasm-function[245]:80)
[00:59:02]     at core::result::unwrap_failed::hb8e1b756b0aad0f7 (wasm-function[19]:149)
[00:59:02]     at _$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$::unwrap::h047f9d6689dc96bf (wasm-function[1]:43)
[00:59:02]     at dbg_macro_expected_behavior::main::h80c10857552d47ce (wasm-function[6]:2332)
[00:59:02]     at std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hffb7cd8ec38eb3e9 (wasm-function[9]:25)
[00:59:02]     at std::sys_common::backtrace::__rust_begin_short_backtrace::h178ce1cdbcfd3352 (wasm-function[68]:8)
[00:59:02]     at std::panicking::try::do_call::h92ed3f8ab1f1fe4c (.llvm.7229658430215002035) (wasm-function[134]:20)
[00:59:02]     at __rust_maybe_catch_panic (wasm-function[147]:5)
[00:59:02]     at std::rt::lang_start_internal::had2eaac6c6c88dfd (wasm-function[69]:150)
[00:59:02]     at std::rt::lang_start::hecee5b62d0f0e846 (wasm-function[8]:42)
[00:59:02]     at main (wasm-function[7]:11)
[00:59:02]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:136:20)
[00:59:02]     at Module._compile (module.js:641:30)
[00:59:02]     at Object.Module._extensions..js (module.js:652:10)
[00:59:02]     at Module.load (module.js:560:32)
[00:59:02] ------------------------------------------
[00:59:02] 
[00:59:02] 
[00:59:02] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:59:02] 
[00:59:02] 
[00:59:02] failures:
[00:59:02] failures:
[00:59:02]     [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs
[00:59:02] test result: FAILED. 6587 passed; 1 failed; 224 ignored; 0 measured; 0 filtered out
[00:59:02] 
[00:59:02] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:59:02] 
[00:59:02] 
[00:59:02] 
[00:59:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:02] 
[00:59:02] 
[00:59:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:59:02] Build completed unsuccessfully in 0:54:57
---
travis_time:end:16074f48:start=1537554224518994378,finish=1537554224525540699,duration=6546321
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20bf28bc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c936540
travis_time:start:0c936540
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10900ca4
$ dmesg | grep -i kill
