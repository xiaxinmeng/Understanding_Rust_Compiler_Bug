plain
[01:17:42] failures:
[01:17:42] 
[01:17:42] ---- [codegen] codegen/enum-debug-niche-2.rs stdout ----
[01:17:42] 
[01:17:42] error: verification with 'FileCheck' failed
[01:17:42] status: exit code: 1
[01:17:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-debug-niche-2/enum-debug-niche-2.ll" "/checkout/src/test/codegen/enum-debug-niche-2.rs"
[01:17:42] ------------------------------------------
[01:17:42] 
[01:17:42] ------------------------------------------
[01:17:42] stderr:
[01:17:42] stderr:
[01:17:42] ------------------------------------------
[01:17:42] /checkout/src/test/codegen/enum-debug-niche-2.rs:12:11: error: CHECK: expected string not found in input
[01:17:42] // CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "Placeholder",{{.*}}extraData: i64 4294967295{{[,)].*}}
[01:17:42]           ^
[01:17:42] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-debug-niche-2/enum-debug-niche-2.ll:379:1: note: scanning from here
[01:17:42] !138 = !{!139, !154, !159}
[01:17:42] 
[01:17:42] ------------------------------------------
[01:17:42] 
[01:17:42] 
[01:17:42] 
[01:17:42] ---- [codegen] codegen/repr-u128.rs stdout ----
[01:17:42] 
[01:17:42] error: verification with 'FileCheck' failed
[01:17:42] status: exit code: 1
[01:17:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-u128/repr-u128.ll" "/checkout/src/test/codegen/repr-u128.rs"
[01:17:42] ------------------------------------------
[01:17:42] 
[01:17:42] ------------------------------------------
[01:17:42] stderr:
[01:17:42] stderr:
[01:17:42] ------------------------------------------
[01:17:42] /checkout/src/test/codegen/repr-u128.rs:15:11: error: CHECK: expected string not found in input
[01:17:42] // CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "None",{{.*}}extraData:18446745000000000124
[01:17:42]           ^
[01:17:42] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-u128/repr-u128.ll:1:1: note: scanning from here
[01:17:42] ; ModuleID = 'repr_u128.7rcbfp3g-cgu.0'
[01:17:42] 
[01:17:42] ------------------------------------------
[01:17:42] 
[01:17:42] 
---
[01:17:42] test result: FAILED. 122 passed; 2 failed; 16 ignored; 0 measured; 0 filtered out
[01:17:42] 
[01:17:42] 
[01:17:42] 
[01:17:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "codegen" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:42] 
[01:17:42] 
[01:17:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[01:17:42] Build completed unsuccessfully in 1:14:26
---
travis_time:end:0044ce56:start=1556746117973389801,finish=1556746117981004654,duration=7614853
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:034ad564
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bb31450
travis_time:start:1bb31450
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1afd9cd8
$ dmesg | grep -i kill
