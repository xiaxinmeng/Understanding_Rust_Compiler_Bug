plain
[00:54:16] ---- [run-make] run-make/wasm-panic-small stdout ----
---
[00:54:16] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small.stage2-wasm32-unknown-unknown:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small.stage2-wasm32-unknown-unknown -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small.stage2-wasm32-unknown-unknown  foo.rs -C lto -O --target wasm32-unknown-unknown
[00:54:16] wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small.stage2-wasm32-unknown-unknown/foo.wasm
[00:54:16] 61468
[00:54:16] [ "`wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small.stage2-wasm32-unknown-unknown/foo.wasm`" -lt "1024" ]
[00:54:16] Makefile:5: recipe for target 'all' failed
[00:54:16]
[00:54:16] ------------------------------------------
[00:54:16] stderr:
[00:54:16] ------------------------------------------
[00:54:16] make: *** [all] Error 1
[00:54:16]
[00:54:16] ------------------------------------------
[00:54:16]
[00:54:16] thread '[run-make] run-make/wasm-panic-small' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
---
[00:54:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-make" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
---
Sun Apr  1 00:20:20 UTC 2018
Sun, 01 Apr 2018 00:20:20 GMT
---
$ cat obj/tmp/sccache.log
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:05756d0c:start=1522542022099358201,finish=1522542022107292198,duration=7933997
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0557f7bd
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0557f7bd:start=1522542022113705545,finish=1522542022121532385,duration=7826840
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f914dcf
$ dmesg | grep -i kill
[   12.286290] init: failsafe main process (1092) killed by TERM signal
