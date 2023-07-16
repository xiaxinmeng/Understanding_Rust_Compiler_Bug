rust
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:12] 
[01:00:12] running 54 tests
[01:00:16] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:329:21
[01:00:16] i...i.ii...i.............i.....ii......i...i..i..i.F..
[01:00:16] failures:
[01:00:16] 
[01:00:16] ---- [codegen] codegen/unchecked-float-casts.rs stdout ----
[01:00:16] 	
[01:00:16] error: verification with 'FileCheck' failed
[01:00:16] status: exit code: 1
[01:00:16] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked-float-casts.ll" "/checkout/src/test/codegen/unchecked-float-casts.rs"
[01:00:16] stdout:
[01:00:16] ------------------------------------------
[01:00:16] 
[01:00:16] ------------------------------------------
[01:00:16] stderr:
[01:00:16] ------------------------------------------
[01:00:16] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked-float-casts.ll:39:7: error: CHECK-NOT: string occurred!
[01:00:16]  %2 = select i1 %0, float 0x7FF0000000000000, float %1
[01:00:16]       ^
[01:00:16] /checkout/src/test/codegen/unchecked-float-casts.rs:63:16: note: CHECK-NOT: pattern specified here
[01:00:16]  // CHECK-NOT: select
[01:00:16]                ^
[01:00:16] 
[01:00:16] ------------------------------------------
[01:00:16] 
[01:00:16] thread '[codegen] codegen/unchecked-float-casts.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2499:8
