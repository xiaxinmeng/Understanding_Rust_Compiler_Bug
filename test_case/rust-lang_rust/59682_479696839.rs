plain
[01:23:20] ---- [run-pass] run-pass/issue-59020.rs stdout ----
[01:23:20] 
[01:23:20] error: test run failed!
[01:23:20] status: exit code: 101
[01:23:20] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-59020/a.wasm"
[01:23:20] ------------------------------------------
[01:23:20] 
[01:23:20] ------------------------------------------
[01:23:20] stderr:
[01:23:20] stderr:
[01:23:20] ------------------------------------------
[01:23:20] RuntimeError: unreachable
[01:23:20]     at __rust_start_panic (wasm-function[123]:1)
[01:23:20]     at rust_panic (wasm-function[116]:39)
[01:23:20]     at _ZN3std9panicking20rust_panic_with_hook17hf29f358d012886a0E (wasm-function[111]:346)
[01:23:20]     at _ZN3std9panicking18continue_panic_fmt17h2ae2deee0bc74311E (wasm-function[110]:151)
[01:23:20]     at rust_begin_unwind (wasm-function[109]:3)
[01:23:20]     at _ZN4core9panicking9panic_fmt17h4bee53922254e4d1E (wasm-function[143]:80)
[01:23:20]     at _ZN4core6result13unwrap_failed17h0aeaf122db551f96E (wasm-function[11]:122)
[01:23:20]     at _ZN3std6thread5spawn17h94cd7e7a502c98a9E (wasm-function[13]:429)
[01:23:20]     at _ZN11issue_590204main17h83cdb8610bc7a399E (wasm-function[18]:25)
[01:23:20]     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h91213904f5139701E (wasm-function[3]:25)
[01:23:20]     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hb3afe3cca1252c5cE (wasm-function[95]:8)
[01:23:20]     at _ZN3std9panicking3try7do_call17h7851f3235948e44fE (wasm-function[108]:20)
[01:23:20]     at __rust_maybe_catch_panic (wasm-function[122]:5)
[01:23:20]     at _ZN3std2rt19lang_start_internal17h73574eba035ece3bE (wasm-function[117]:270)
[01:23:20]     at main (wasm-function[19]:46)
[01:23:20]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:126:20)
[01:23:20]     at Module._compile (module.js:641:30)
[01:23:20]     at Object.Module._extensions..js (module.js:652:10)
[01:23:20]     at Module.load (module.js:560:32)
[01:23:20]     at tryModuleLoad (module.js:503:12)
[01:23:20] ------------------------------------------
[01:23:20] 
[01:23:20] thread '[run-pass] run-pass/issue-59020.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:23:20] 
[01:23:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:23:20] 
[01:23:20] 
[01:23:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-pass" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:20] 
[01:23:20] 
[01:23:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:23:20] Build completed unsuccessfully in 1:10:46
---
travis_time:end:0ae38131:start=1554334832235306147,finish=1554334832253167827,duration=17861680
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01512342
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:105941e0
travis_time:start:105941e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1857ced4
$ dmesg | grep -i kill
