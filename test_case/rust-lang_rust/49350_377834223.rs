plain
[01:09:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros-in-extern.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=wasm32-unknown-unknown" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.stage2-wasm32-unknown-unknown.wasm" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.stage2-wasm32-unknown-unknown.aux"
---
[01:09:06] error: linking with `lld` failed: exit code: 1
[01:09:06]   |
[01:09:06]   = note: "lld" "-flavor" "wasm" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.macros_in_extern0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.macros_in_extern1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.macros_in_extern2-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.stage2-wasm32-unknown-unknown.wasm" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.crate.allocator.rcgu.o" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros-in-extern.stage2-wasm32-unknown-unknown.aux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "-l" "rust_test_helpers" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libstd-dbbd850df25e22d6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libpanic_abort-95f1fae8714ef10b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libunwind-aa0e4e666207dc49.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/liblibc-70db4fd830eb0511.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/liballoc_system-f7a9cbc4f22ce580.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libdlmalloc-2e21ae4bd29a5dae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/liballoc-2213efc3149321df.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libstd_unicode-f6f336308541ac07.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libcore-6db47a0e2830c329.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libcompiler_builtins-3c1279ab78e3cad2.rlib" "--threads" "--allow-undefined" "--no-entry"
[01:09:06]   = note: lld: error: unknown file type: rust_test_helpers.o
---
[01:09:06] thread '[run-pass] run-pass/macros-in-extern.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
---
[01:09:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-pass" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
---
$ cat obj/tmp/sccache.log
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:01f882aa:start=1522633045198067563,finish=1522633045206794349,duration=8726786
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2acf0b40
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2acf0b40:start=1522633045214077533,finish=1522633045222534461,duration=8456928
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f2184f
$ dmesg | grep -i kill
[   11.847813] init: failsafe main process (1095) killed by TERM signal
