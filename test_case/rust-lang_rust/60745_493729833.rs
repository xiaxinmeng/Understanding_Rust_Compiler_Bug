plain
[01:12:58] ---- [mir-opt] mir-opt/const_prop/switch_int.rs stdout ----
[01:12:58] 
[01:12:58] error: test run failed!
[01:12:58] status: exit code: 101
[01:12:58] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/switch_int/a.wasm"
[01:12:58] ------------------------------------------
[01:12:58] 
[01:12:58] ------------------------------------------
[01:12:58] stderr:
[01:12:58] stderr:
[01:12:58] ------------------------------------------
[01:12:58] RuntimeError: unreachable
[01:12:58]     at _ZN3std7process4exit17h51f592b24e75ba54E (wasm-function[45]:7)
[01:12:58]     at _ZN10switch_int4main17h111c63dfc6bd55dfE (wasm-function[4]:3)
[01:12:58]     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h7c60c3b15001320bE (wasm-function[1]:25)
[01:12:58]     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h0e4d065d7831dc3cE (wasm-function[49]:8)
[01:12:58]     at _ZN3std9panicking3try7do_call17h0ec38f4cd0f182c0E (wasm-function[57]:20)
[01:12:58]     at __rust_maybe_catch_panic (wasm-function[68]:5)
[01:12:58]     at _ZN3std2rt19lang_start_internal17h2b50dd71369ffc77E (wasm-function[66]:270)
[01:12:58]     at main (wasm-function[5]:46)
[01:12:58]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:126:20)
[01:12:58]     at Module._compile (module.js:641:30)
[01:12:58]     at Object.Module._extensions..js (module.js:652:10)
[01:12:58]     at Module.load (module.js:560:32)
[01:12:58]     at tryModuleLoad (module.js:503:12)
[01:12:58]     at Function.Module._load (module.js:495:3)
[01:12:58]     at Function.Module.runMain (module.js:682:10)
[01:12:58]     at startup (bootstrap_node.js:191:16)
[01:12:58]     at bootstrap_node.js:613:3
[01:12:58] ------------------------------------------
[01:12:58] 
[01:12:58] 
[01:12:58] 
[01:12:58] 
[01:12:58] failures:
[01:12:58]     [mir-opt] mir-opt/const_prop/switch_int.rs
[01:12:58] 
[01:12:58] test result: FAILED. 41 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
[01:12:58] 
[01:12:58] 
[01:12:58] 
[01:12:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:58] 
[01:12:58] 
[01:12:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:12:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
travis_time:end:075eec48:start=1558245877163809992,finish=1558245877172679354,duration=8869362
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f8bfef4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20f6d0fc
travis_time:start:20f6d0fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:196062d5
$ dmesg | grep -i kill
