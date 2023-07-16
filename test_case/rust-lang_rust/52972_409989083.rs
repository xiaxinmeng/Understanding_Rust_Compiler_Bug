plain
[00:57:25] failures:
[00:57:25] 
[00:57:25] ---- [codegen] codegen/vec-clear.rs stdout ----
[00:57:25] 
[00:57:25] error: verification with 'FileCheck' failed
[00:57:25] status: exit code: 1
[00:57:25] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll" "/checkout/src/test/codegen/vec-clear.rs"
[00:57:25] ------------------------------------------
[00:57:25] 
[00:57:25] ------------------------------------------
[00:57:25] stderr:
[00:57:25] stderr:
[00:57:25] ------------------------------------------
[00:57:25] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll:18:7: error: CHECK-NOT: string occurred!
[00:57:25]  %1 = load i64, i64* %0, align 8, !range !0, !alias.scope !1
[00:57:25]       ^
[00:57:25] /checkout/src/test/codegen/vec-clear.rs:18:16: note: CHECK-NOT: pattern specified here
[00:57:25]  // CHECK-NOT: load
[00:57:25] 
[00:57:25] ------------------------------------------
[00:57:25] 
[00:57:25] thread '[codegen] codegen/vec-clear.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
---
[00:57:25] test result: FAILED. 74 passed; 1 failed; 27 ignored; 0 measured; 0 filtered out
[00:57:25] 
[00:57:25] 
[00:57:25] 
[00:57:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
