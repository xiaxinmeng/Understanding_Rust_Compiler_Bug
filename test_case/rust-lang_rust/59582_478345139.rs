plain
[01:28:44] failures:
[01:28:44] 
[01:28:44] ---- [codegen] codegen/c-variadic.rs stdout ----
[01:28:44] 
[01:28:44] error: verification with 'FileCheck' failed
[01:28:44] status: exit code: 1
[01:28:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll" "/checkout/src/test/codegen/c-variadic.rs"
[01:28:44] ------------------------------------------
[01:28:44] 
[01:28:44] ------------------------------------------
[01:28:44] stderr:
[01:28:44] stderr:
[01:28:44] ------------------------------------------
[01:28:44] /checkout/src/test/codegen/c-variadic.rs:77:12: error: CHECK: expected string not found in input
[01:28:44]  // CHECK: call void (i32, ...) @foreign_c_variadic_0(i32 0, { i64, i64 } %{{.*}}, %Bar* byval noalias nocapture dereferenceable(24) %{{.*}})
[01:28:44]            ^
[01:28:44]            ^
[01:28:44] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll:1066:2: note: scanning from here
[01:28:44]  br label %bb4
[01:28:44] 
[01:28:44] ------------------------------------------
[01:28:44] 
[01:28:44] thread '[codegen] codegen/c-variadic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:28:44] test result: FAILED. 119 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out
[01:28:44] 
[01:28:44] 
[01:28:44] 
[01:28:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "codegen" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:28:44] 
[01:28:44] 
[01:28:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[01:28:44] Build completed unsuccessfully in 1:25:58
---
travis_time:end:0ae8b1e0:start=1554041175929839217,finish=1554041175936917704,duration=7078487
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:001aa11f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0228e220
travis_time:start:0228e220
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:051d6c79
$ dmesg | grep -i kill
