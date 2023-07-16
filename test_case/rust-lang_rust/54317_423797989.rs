plain
[00:58:20] test [ui] ui/rfc-2306/convert-id-const-no-gate.rs ... ok
[00:58:20] test [ui] ui/rfc-2126-extern-absolute-paths/single-segment.rs ... ok
[00:58:20] test [ui] ui/rfc-2166-underscore-imports/basic.rs ... ok
[00:58:20] test [ui] ui/rfc-2126-extern-in-paths/single-segment.rs ... ok
[00:58:20] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-feature-gate.rs ... ok
[00:58:20] test [ui] ui/rfc-2306/convert-id-const-with-gate.rs ... ok
[00:58:21] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs ... ok
[00:58:21] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs ... ok
[00:58:21] test [ui] ui/rfc1445/feature-gate.rs#no_gate ... ok
[00:58:21] test [ui] ui/rfc-2497-if-let-chains/syntax-ambiguity-2018.rs ... ok
[00:58:21] test [ui] ui/rfc1445/feature-gate.rs#with_gate ... ok
[00:58:21] test [ui] ui/rfc1445/match-forbidden-without-eq.rs ... ok
---
[00:58:22] test [ui] ui/rmeta-priv-warn.rs ... ok
[00:58:22] test [ui] ui/rmeta-lib-pass.rs ... ok
[00:58:22] test [ui] ui/rmeta-pass.rs ... ok
[00:58:22] test [ui] ui/rmeta.rs ... ok
[00:58:22] test [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs ... FAILED
[00:58:22] test [ui] ui/rmeta_meta_main.rs ... ok
[00:58:23] test [ui] ui/run-pass/array-slice-vec/arr_cycle.rs ... ok
[00:58:23] test [ui] ui/run-pass/allocator/custom.rs ... ok
[00:58:23] test [ui] ui/run-pass/array-slice-vec/box-of-array-of-drop-1.rs ... ignored
---
[01:04:29] test [ui] ui/xcrate/xcrate-unit-struct.rs ... ok
[01:04:29] 
[01:04:29] failures:
[01:04:29] 
[01:04:29] ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
[01:04:29] error: test run failed!
[01:04:29] status: exit code: 101
[01:04:29] status: exit code: 101
[01:04:29] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a.wasm"
[01:04:29] ------------------------------------------
[01:04:29] 
[01:04:29] ------------------------------------------
[01:04:29] stderr:
[01:04:29] stderr:
[01:04:29] ------------------------------------------
[01:04:29] RuntimeError: unreachable
[01:04:29]     at __rust_start_panic (wasm-function[154]:1)
[01:04:29]     at rust_panic.llvm.9030642565800848782 (wasm-function[149]:38)
[01:04:29]     at std::panicking::rust_panic_with_hook::h226a58a5977f2e17 (wasm-function[144]:500)
[01:04:29]     at std::panicking::continue_panic_fmt::h5f0126bd5d9da93f (wasm-function[143]:155)
[01:04:29]     at rust_begin_unwind (wasm-function[142]:3)
[01:04:29]     at core::panicking::panic_fmt::hebc42c0af7746402 (wasm-function[252]:80)
[01:04:29]     at core::result::unwrap_failed::hb8e1b756b0aad0f7 (wasm-function[19]:149)
[01:04:29]     at _$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$::unwrap::h047f9d6689dc96bf (wasm-function[1]:43)
[01:04:29]     at dbg_macro_expected_behavior::main::h80c10857552d47ce (wasm-function[6]:2332)
[01:04:29]     at std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hffb7cd8ec38eb3e9 (wasm-function[9]:25)
[01:04:29]     at std::sys_common::backtrace::__rust_begin_short_backtrace::h178ce1cdbcfd3352 (wasm-function[76]:8)
[01:04:29]     at std::panicking::try::do_call::h92ed3f8ab1f1fe4c (.llvm.9030642565800848782) (wasm-function[141]:20)
[01:04:29]     at __rust_maybe_catch_panic (wasm-function[153]:5)
[01:04:29]     at std::rt::lang_start_internal::had2eaac6c6c88dfd (wasm-function[74]:128)
[01:04:29]     at std::rt::lang_start::hecee5b62d0f0e846 (wasm-function[8]:42)
[01:04:29]     at main (wasm-function[7]:11)
[01:04:29]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:136:20)
[01:04:29]     at Module._compile (module.js:641:30)
[01:04:29]     at Object.Module._extensions..js (module.js:652:10)
[01:04:29]     at Module.load (module.js:560:32)
[01:04:29] ------------------------------------------
[01:04:29] 
[01:04:29] 
[01:04:29] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:04:29] 
[01:04:29] 
[01:04:29] failures:
[01:04:29] failures:
[01:04:29]     [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs
[01:04:29] test result: FAILED. 6589 passed; 1 failed; 224 ignored; 0 measured; 0 filtered out
[01:04:29] 
[01:04:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:04:29] 
[01:04:29] 
[01:04:29] 
[01:04:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:29] 
[01:04:29] 
[01:04:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:04:29] Build completed unsuccessfully in 1:00:35
---
travis_time:end:123b7060:start=1537688212272112198,finish=1537688212275900531,duration=3788333
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:004b20aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0357c60e
travis_time:start:0357c60e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cf7f29e
$ dmesg | grep -i kill
